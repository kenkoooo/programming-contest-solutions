import java.util.Scanner

object ACPC2017Day2G {
  def main(args: Array[String]): Unit = {
    val in = new Scanner(System.in)
    val N = in.nextInt()
    val X = in.nextInt()
    val Y = in.nextInt()
    val foods = (for (_ <- 0 until N) yield {
      val K = in.nextInt()
      (for (_ <- 0 until K) yield {
        val a = in.nextInt()
        val b = in.nextInt()
        val c = in.nextInt()
        (a, b, c)
      }).toArray
    }).toArray

    val dist = (for (_ <- 0 until N) yield (for (_ <- 0 until N) yield in.nextInt()).toArray).toArray
    for {
      k <- 0 until N
      i <- 0 until N
      j <- 0 until N
    } dist(i)(j) = math.min(dist(i)(j), dist(i)(k) + dist(k)(j))

    val cost = Array.fill(1 << (N + 1), N + 1)(Int.MaxValue / 2)
    cost(1)(0) = 0
    for {
      mask <- 0 until (1 << N)
      current <- 0 until N
      next <- 1 until (N + 1)
      if (mask & (1 << current)) != 0
      if (mask & (1 << next)) == 0
    } {
      val nextMask = mask | (1 << next)
      cost(nextMask)(next) = math.min(cost(nextMask)(next), cost(mask)(current) + dist(current)(next % N))
    }

    val bottom = N / 2
    val top = N - bottom

    val deq = new Array[Int](1000000)
    val deqv = new Array[Int](1000000)

    def knapsack(mask: Int, dp: Array[Int]): Unit = {
      dp(0) = 0
      for {
        i <- 0 until N
        if (mask & (1 << i)) != 0
        (price, value, remain) <- foods(i)
        a <- 0 until price
      } {
        var s = 0
        var t = 0

        var num = 0
        while (num * price <= (Y - a)) {
          val v = dp(num * price + a) - num * value
          while (s < t && deqv(t - 1) <= v) {
            t -= 1
          }
          deq(t) = num
          deqv(t) = v
          t += 1

          dp(num * price + a) = deqv(s) + num * value
          if (deq(s) == num - remain) {
            s += 1
          }

          num += 1
        }
      }

      for {
        i <- 1 to Y
      } {
        dp(i) = math.max(dp(i), dp(i - 1))
      }
    }

    val bottomDp = Array.fill[Int](1 << bottom, Y + 1)(-1)
    val topDp = Array.fill[Int](1 << top, Y + 1)(-1)
    for (mask <- 0 until (1 << bottom)) {
      knapsack(mask, bottomDp(mask))
    }
    for (mask <- 0 until (1 << top)) {
      knapsack(mask << bottom, topDp(mask))
    }

    var max = 0
    for {
      mask <- 0 until (1 << N)
    } {
      val moveCost = cost(mask | (1 << N))(N)
      val budget = math.min(Y, X - moveCost)
      val bottomMask = mask & ((1 << bottom) - 1)
      val topMask = mask >> bottom

      if (budget > 0) {
        for {
          bottomBudget <- 0 to budget
        } {
          max = math.max(max, bottomDp(bottomMask)(bottomBudget) + topDp(topMask)(budget - bottomBudget))
        }
      }
    }
    println(max)
  }
}