import java.util.Scanner

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

object ACPC2017Day1E extends App {
  val in = new Scanner(System.in)
  val V = in.nextInt()
  val E = in.nextInt()
  val dinitz1 = new Dinitz(V)
  val dinitz2 = new Dinitz(V)
  for (_ <- 0 until E) {
    val s = in.nextInt()
    val t = in.nextInt()
    val c = in.nextInt()
    dinitz1.addEdge(s, t, c)
    dinitz1.addEdge(t, s, c)
    dinitz2.addEdge(s, t, c)
    dinitz2.addEdge(t, s, c)
  }

  val f1 = dinitz1.maxFlow(0, V - 1)
  for {
    i <- dinitz1.graph.indices
    j <- dinitz1.graph(i).indices
    if dinitz1.graph(i)(j).cap == 0
    if dinitz2.graph(i)(j).cap == 1
  } {
    val e1 = dinitz2.graph(i)(j)
    val e2 = dinitz2.graph(e1.to)(e1.rev)
    e1.cap = 0
    e2.cap = 0
  }

  val f2 = dinitz2.maxFlow(0, V - 1)
  val flow = if (f1 > f2) {
    f1 - 1
  } else {
    f1
  }
  if (flow > 10000) {
    println(-1)
  } else {
    println(flow)
  }
}

class Dinitz(val V: Int) {

  class Edge(val to: Int, var cap: Long, val rev: Int)

  val graph: Array[ArrayBuffer[Edge]] = (for (_ <- 0 until V) yield new ArrayBuffer[Edge]()).toArray

  def addEdge(from: Int, to: Int, cap: Long): Unit = {
    graph(from).append(new Edge(to, cap, graph(to).size))
    graph(to).append(new Edge(from, 0, graph(from).size - 1))
  }

  def maxFlow(source: Int, sink: Int): Long = {
    var flow = 0L

    while (true) {
      val queue = new mutable.Queue[Int]()
      val level = Array.fill[Int](V)(-1)
      level(source) = 0
      queue.enqueue(source)
      while (queue.nonEmpty) {
        val v = queue.dequeue()
        for (e <- graph(v)) {
          if (e.cap > 0 && level(e.to) < 0) {
            level(e.to) = level(v) + 1
            queue.enqueue(e.to)
          }
        }
      }

      if (level(sink) < 0) return flow
      val iter = new Array[Int](V)

      def dfs(v: Int, t: Int, f: Long): Long = {
        if (v == t) return f

        while (iter(v) < graph(v).size) {
          val e = graph(v)(iter(v))
          if (e.cap > 0 && level(v) < level(e.to)) {
            val d = dfs(e.to, t, Math.min(f, e.cap))
            if (d > 0) {
              e.cap -= d
              graph(e.to)(e.rev).cap += d
              return d
            }
          }

          iter(v) += 1
        }
        0
      }

      var f = 1L
      while (f > 0) {
        f = dfs(source, sink, Long.MaxValue)
        flow += f
      }
    }

    throw new IllegalStateException()
  }
}