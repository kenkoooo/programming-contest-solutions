import java.util.Scanner

import scala.collection.immutable.Stack
import scala.collection.mutable.ArrayBuffer

object GRL_3_C {
  def main(args: Array[String]): Unit = {
    val in = new Scanner(System.in)
    val v = in.nextInt()
    val graph = (0 until v).map { _ => new ArrayBuffer[Int]() }.toArray
    val e = in.nextInt()
    for (_ <- 0 until e) {
      val s = in.nextInt()
      val t = in.nextInt()
      graph(s).append(t)
    }

    val cmp = StronglyConnectedComponents.decompose(graph)
    val q = in.nextInt()
    for (_ <- 0 until q) {
      val u = in.nextInt()
      val v = in.nextInt()
      if (cmp(u) == cmp(v)) {
        println(1)
      } else {
        println(0)
      }
    }
  }

  object StronglyConnectedComponents {
    def decompose(graph: IndexedSeq[Seq[Int]]): Array[Int] = {
      val vs = new ArrayBuffer[Int]()
      val V = graph.size
      val cmp = new Array[Int](V)

      val rg = graph.indices.map(_ => new ArrayBuffer[Int]())
      for {
        from <- graph.indices
        to <- graph(from)
      } rg(to).append(from)

      var used = Array.fill[Boolean](V)(false)
      var stack = new Stack[Int]()
      val added = Array.fill[Boolean](V)(false)
      for {
        i <- used.indices
        if !used(i)
      } {
        stack = stack.push(i)
        while (stack.nonEmpty) {
          val v = stack.top
          used(v) = true
          var pushed = false
          for {
            u <- graph(v).reverse
            if !used(u)
          } {
            stack = stack.push(u)
            pushed = true
          }
          if (!pushed) {
            stack = stack.pop
            if (!added(v)) {
              vs.append(v)
              added(v) = true
            }
          }
        }
      }

      used = Array.fill[Boolean](V)(false)
      var k = 0
      for {
        i <- vs.reverse
        if !used(i)
      } {
        stack = stack.push(i)
        used(i) = true
        cmp(i) = k
        while (stack.nonEmpty) {
          val v = stack.top
          var pushed = false
          for {
            u <- rg(v)
            if !used(u)
          } {
            used(u) = true
            cmp(u) = k
            stack = stack.push(u)
            pushed = true
          }
          if (!pushed) {
            stack = stack.pop
          }
        }
        k += 1
      }

      cmp
    }
  }

}
