import java.util

import scala.io.StdIn

object AOJ3019 extends App {
  val INF: Long = 1e15.toLong
  val (n, x, y) = {
    val in = StdIn.readLine().split(" ").map(_.toInt)
    (in(0), in(1), in(2))
  }
  val sweets = for (_ <- 0 until n) yield {
    val k = StdIn.readLine().toInt
    for (_ <- 0 until k) yield {
      val in = StdIn.readLine().split(" ").map(_.toInt)
      Sweet(in(0), in(1), in(2))
    }
  }

  val dist = for (_ <- 0 until n) yield StdIn.readLine().split(" ").map(_.toLong)
  for (k <- 0 until n) for (i <- 0 until n) for (j <- 0 until n) dist(i)(j) = math.min(dist(i)(j), dist(i)(k) + dist(k)(j))

  val cost = Array.fill[Long](1 << n, n)(INF)
  cost(0)(0) = 0
  for (mask <- 0 until 1 << n)
    for (cur <- 0 until n)
      for (next <- 0 until n)
        if ((mask & (1 << next)) == 0) {
          val add = dist(cur)(next)
          cost(mask | (1 << next))(next) = math.min(cost(mask | (1 << next))(next), cost(mask)(cur) + add)
        }


  def knapsack(dp: Array[Long], available: Seq[Sweet]): Unit = {
    for {
      sweet <- available
      money <- 0 until sweet.price
    } {
      val deque = new util.ArrayDeque[(Int, Long)]()
      var num = 0
      while (num * sweet.price + money <= y) {
        val next = num * sweet.price + money


        val v = dp(next) - num * sweet.value
        while (!deque.isEmpty && deque.peekLast()._2 <= v) deque.pollLast()
        deque.addLast((num, v))

        dp(next) = deque.peekFirst()._2 + num * sweet.value
        if (deque.peekFirst()._1 == num - sweet.remain) deque.pollFirst()

        num += 1
      }
    }
    for (i <- 1 until dp.length) dp(i) = math.max(dp(i), dp(i - 1))
  }


  val prefix = n / 2
  val suffix = n - prefix

  val prefixDp = Array.fill[Long](1 << prefix, y + 1)(0)
  for (prefixMask <- 0 until 1 << prefix) {
    val mask = prefixMask << suffix
    val dp = prefixDp(prefixMask)
    val available = for {
      i <- 0 until n
      if (mask & (1 << i)) != 0
      sweet <- sweets(i)
    } yield sweet
    knapsack(dp, available)
  }

  val suffixDp = Array.fill[Long](1 << suffix, y + 1)(0)
  for (mask <- 0 until 1 << suffix) {
    val dp = suffixDp(mask)
    val available = for {
      i <- 0 until n
      if (mask & (1 << i)) != 0
      sweet <- sweets(i)
    } yield sweet
    knapsack(dp, available)
  }
  var ans = 0L
  for (mask <- 0 until 1 << n) {
    val suffixMask = mask & ((1 << suffix) - 1)
    val prefixMask = mask >> suffix
    val travelingCost = (for (last <- 0 until n) yield {
      cost(mask)(last) + dist(last)(0)
    }).min
    val money = math.min(x - travelingCost, y).toInt
    for (prefixMoney <- 0 to money) {
      val suffixMoney = money - prefixMoney
      val value = prefixDp(prefixMask)(prefixMoney) + suffixDp(suffixMask)(suffixMoney)
      ans = math.max(ans, value)
    }
  }
  println(ans)
}

case class Sweet(price: Int, value: Int, remain: Int)