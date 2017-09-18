import java.util.Scanner

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

object AOJ2828 extends App {
  val in = new Scanner(System.in)

  def solve(n: Int): Long = {
    val xyz = (for (_ <- 0 until n) yield {
      Array(in.nextInt(), in.nextInt(), in.nextInt()).sorted
    }).toArray


    val source = n * 2
    val sink = source + 1
    val minimumCostFlow = new MinimumCostFlow(sink + 1)
    for {
      i <- 0 until n
      j <- 0 until n
      if i != j
      if xyz(i)(0) < xyz(j)(0)
      if xyz(i)(1) < xyz(j)(1)
      if xyz(i)(2) < xyz(j)(2)
    } {
      minimumCostFlow.addEdge(i, n + j, 1, 0)
    }
    for (i <- 0 until n) {
      minimumCostFlow.addEdge(source, i, 1, 0)
      minimumCostFlow.addEdge(i, sink, 1, xyz(i)(0) * xyz(i)(1) * xyz(i)(2))
      minimumCostFlow.addEdge(i + n, sink, 1, 0)
    }
    minimumCostFlow.calculateCost(source, sink, n)
  }

  def loop(): Unit = {
    while (true) {
      val n = in.nextInt()
      if (n == 0) {
        return
      }
      val ans = solve(n)
      println(ans)
    }
  }

  loop()

}


class MinimumCostFlow(V: Int) {

  class Edge(val to: Int, var cap: Long, val cost: Long, val rev: Int)

  val graph: Array[ArrayBuffer[Edge]] = (for (_ <- 0 until V) yield new ArrayBuffer[Edge]()).toArray
  val prevV = new Array[Int](V)
  val prevE = new Array[Int](V)

  def addEdge(from: Int, to: Int, cap: Long, cost: Long): Unit = {
    graph(from).append(new Edge(to, cap, cost, graph(to).size))
    graph(to).append(new Edge(from, 0, -cost, graph(from).size - 1))
  }

  def calculateCost(source: Int, sink: Int, flow: Long): Long = {
    val INF = Long.MaxValue / 2
    var cost = 0L
    var residue = flow
    val potential = new Array[Long](V)
    while (residue > 0) {
      val dist = Array.fill[Long](V)(INF)
      dist(source) = 0
      val queue = mutable.PriorityQueue.empty[(Long, Int)](implicitly[Ordering[(Long, Int)]].reverse)
      queue.enqueue((0, source))
      while (queue.nonEmpty) {
        val p = queue.dequeue()
        val v = p._2
        if (dist(v) >= p._1) {
          for (i <- graph(v).indices) {
            val e = graph(v)(i)
            val u = e.to
            if (e.cap > 0 && dist(u) > dist(v) + e.cost + potential(v) - potential(u)) {
              dist(u) = dist(v) + e.cost + potential(v) - potential(u)
              prevV(u) = v
              prevE(u) = i
              queue.enqueue((dist(u), u))
            }
          }
        }
      }
      if (dist(sink) == INF) {
        return -1
      }

      for (v <- 0 until V) {
        potential(v) += dist(v)
      }
      var d = residue
      var v = sink
      while (v != source) {
        d = math.min(d, graph(prevV(v))(prevE(v)).cap)
        v = prevV(v)
      }
      residue -= d
      cost += d * potential(sink)

      v = sink
      while (v != source) {
        val e = graph(prevV(v))(prevE(v))
        e.cap -= d
        graph(v)(e.rev).cap += d
        v = prevV(v)
      }
    }
    return cost
  }
}