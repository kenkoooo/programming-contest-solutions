import java.util.Scanner

object ACPC2017Day2H extends App {
  val in = new Scanner(System.in)
  val N = in.nextInt()
  val L = in.nextInt()
  val K = in.nextLong()
  val ad: Array[(Long, Long)] = (for (_ <- 0 until N) yield {
    val a: Long = in.nextInt()
    val d: Long = in.nextInt()
    (a, d)
  }).toArray

  val sum = ad.map {
    case (a, d) => (a + (a + (L - 1) * d)) * L / 2
  }

  // imos[i+1] = sum(0...i)
  val imos = new Array[Long](N + 1)
  for (i <- 0 until N) {
    imos(i + 1) = imos(i) + sum(i)
  }


  def calc(a0: Long, da: Long, b0: Long, db: Long, p: Int): Long = {
    def value(x: Double): Double = (b0 - a0) * x + x * (x - 1) * (db - da) / 2

    var left = 0.0
    var right = p.toDouble

    for (_ <- 0 to 200) {
      var w = (right - left) / 3
      if (value(left + w) < value(right - w)) {
        left += w
      } else {
        right -= w
      }
    }

    val x = left.toInt
    var max = 0.0
    for {
      i <- x - 10 to x + 10
      if i >= 0 && i <= p
    } {
      max = math.max(max, value(i))
    }
    max.toLong
  }


  if (K == N.toLong * L) {
    println(imos(N))
  } else {
    val ans = (for {
      head <- 0 until N
      if head + K / L < N
    } yield {
      val a0 = ad(head)._1
      val da = ad(head)._2

      val idx = (head + K / L).toInt
      val rest = (K % L).toInt
      val p = L - rest

      val b0 = ad(idx)._1 + rest * ad(idx)._2
      val db = ad(idx)._2

      val sum1 = imos(idx) - imos(head) + (ad(idx)._1 + ad(idx)._1 + (rest - 1) * db) * rest / 2
      val sum2 = calc(a0, da, b0, db, p) + sum1

      if (idx + 1 < N && rest >= 2) {
        val sum3 = sum1 + (b0 + b0 + (p - 1) * db) * p / 2 - (a0 + a0 + (p - 1) * da) * p / 2

        val c0 = ad(idx + 1)._1
        val dc = ad(idx + 1)._2

        val ax = a0 + p * da

        val sum4 = calc(ax, da, c0, dc, rest) + sum3

        math.max(sum2, sum4)
      } else {
        sum2
      }
    }).max
    println(ans)
  }
}
