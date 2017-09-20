import java.util.Scanner

import scala.collection.mutable.ArrayBuffer

object ACPC2017Day3E extends App {
  val in = new Scanner(System.in)
  val H = in.nextInt()
  val W = in.nextInt()
  val T = in.nextInt()
  val Q = in.nextInt()

  var queries = new ArrayBuffer[(Int, Int, Int, Int, Int, Int)]()
  for (_ <- 0 until Q) {
    val t = in.nextInt()
    val c = in.nextInt()
    val h = in.nextInt()
    val w = in.nextInt()
    val (h2, w2) = if (c == 2) {
      (in.nextInt(), in.nextInt())
    } else {
      (0, 0)
    }
    queries.append((t, c, h, w, h2, w2))
    if (c == 0) {
      // 焼き上がり
      queries.append((t + T, -1, h, w, h2, w2))
    }
  }
  queries = queries.sortBy(q => (q._1, q._2))

  val f1 = new Fenwick2D(H, W)
  val f2 = new Fenwick2D(H, W)
  queries.foreach { case (_, c, h, w, h2, w2) =>
    c match {
      case -1 =>
        // 1->2
        if (f1.get(h, w) != 0) {
          f1.update(h, w, 0)
          f2.update(h, w, 1)
        }
      case 0 =>
        // 0->1
        f1.update(h, w, 1)
      case 1 =>
        // 2->0
        f2.update(h, w, 0)
      case 2 =>
        // count
        val a = f2.sum(h, w, h2, w2)
        val b = f1.sum(h, w, h2, w2)
        println(s"$a $b")
    }
  }


}

/**
  * 1-indexed 2-dimensional Fenwick's tree
  *
  * @param h height
  * @param w width
  */
class Fenwick2D(h: Int, w: Int) {
  val N: Int = h + 1
  val M: Int = w + 1
  val data: Array[Array[Long]] = Array.fill[Long](N + 1, M + 1)(0)

  def add(x: Int, y: Int, v: Long): Unit = {
    var i = x
    while (i <= N) {
      var j = y
      while (j <= M) {
        data(i)(j) += v
        j += j & -j
      }
      i += i & -i
    }
  }

  def update(x: Int, y: Int, v: Long): Unit = add(x, y, v - sum(x, y, x, y))

  def get(x: Int, y: Int): Long = sum(x, y, x, y)

  def sum(x0: Int, y0: Int, x1: Int, y1: Int): Long = {

    def partialSum(x: Int, y: Int): Long = {
      var res = 0L
      var i = x
      while (i > 0) {
        var j = y
        while (j > 0) {
          res += data(i)(j)
          j -= j & -j
        }
        i -= i & -i
      }
      res
    }

    partialSum(x1, y1) - partialSum(x0 - 1, y1) - partialSum(x1, y0 - 1) + partialSum(x0 - 1, y0 - 1)
  }


}