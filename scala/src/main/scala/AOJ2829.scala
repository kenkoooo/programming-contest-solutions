import java.util.Scanner

import scala.collection.mutable.ArrayBuffer

object AOJ2829 {
  private val Mod = (1e9 + 7).toInt

  // 階乗を前計算しておく
  private val Fact = {
    var cur = 1L
    for (i <- 0 to 200000) yield {
      if (i == 0) {
        1
      } else {
        cur = (cur * i) % Mod
        cur
      }
    }
  }

  // 2 の累乗を前計算しておく
  private val Pow = {
    val pow = new Array[Int](200000)
    pow(0) = 1
    for (i <- 1 until pow.length) pow(i) = pow(i - 1) * 2 % Mod
    pow
  }

  def main(args: Array[String]): Unit = {
    val in = new Scanner(System.in)
    while (true) {
      val n = in.nextInt()
      if (n == 0) {
        return
      }
      val a = for (_ <- 0 until n) yield {
        in.nextInt() - 1
      }
      println(solve(a.toArray))
    }
  }

  // 長さ 3 以上の閉路が存在しないことを確認する
  def cycleCheck(graph: IndexedSeq[Seq[Int]]): Boolean = {
    val cmp = StronglyConnectedComponents.decompose(graph)
    var map = Map[Int, Int]()
    cmp.foreach { c => map += (c -> (map.getOrElse(c, 0) + 1)) }
    map.values.forall(count => count <= 2)
  }

  // 全ての弱連結成分がパスが閉路を潰すとパスになることを確認する
  def pathCheck(graph: IndexedSeq[Seq[Int]]): Boolean = {
    val uf = new UnionFind(graph.size)
    for {
      i <- graph.indices
      j <- graph(i)
    } uf.unite(i, j)

    var map = Map[Int, Int]()
    for {
      i <- graph.indices
      if graph(i).isEmpty
    } {
      val count = map.getOrElse(uf.find(i), 0)
      map += (uf.find(i) -> (count + 1))
    }
    map.values.forall(count => count <= 2)
  }

  // 要素数 2 の弱連結成分の数と、要素数 3 以上の弱連結成分の数を算出する
  def getSize(array: Array[Int]): (Int, Int) = {
    val N = array.length
    val uf = new UnionFind(N)
    for (i <- array.indices) uf.unite(i, array(i))
    val sizes = for {
      i <- 0 until N
      if i == 0 || !uf.isSame(i, 0)
    } yield {
      if (i == 0) {
        uf.partialSizeOf(i)
      } else {
        val s = uf.partialSizeOf(i)
        uf.unite(i, 0)
        s
      }
    }

    var three = 0
    var two = 0
    sizes.foreach(size => if (size == 2) {
      two += 1
    } else {
      three += 1
    })

    (two, three)
  }

  def solve(parent: Array[Int]): Int = {
    val n = parent.length
    val combination = new Combination(n + 1, Mod)

    val graph = parent.indices.map(i => new ArrayBuffer[Int]())
    parent.indices.foreach(i => graph(parent(i)).append(i))

    if (cycleCheck(graph) && pathCheck(graph)) {
      val (two, three) = getSize(parent)
      if (three == 0) {
        // 長さ 3 以上の弱連結成分が存在しないとき
        var cur: Long = ((two + 2) / 2) % Mod
        cur = cur * Pow(two) % Mod
        cur = cur * Fact(two) % Mod
        cur.toInt
      } else {
        var ans = 0L
        for (i <- 0 to math.min(two + three - 2, two)) {
          var cur = combination.get(two + three - i - 2, two - i).toLong
          cur = cur * Fact(two) % Mod
          cur = cur * Fact(three) % Mod
          cur = cur * Pow(two + three) % Mod
          cur = cur * (two + three - i - 1) % Mod
          cur = cur * ((i + 2) / 2) % Mod
          ans = (ans + cur) % Mod
        }
        for (i <- 0 to math.min(two + three - 1, two)) {
          var cur = combination.get(two + three - i - 1, two - i).toLong
          cur = cur * Fact(two) % Mod
          cur = cur * Fact(three) % Mod
          cur = cur * Pow(two + three) % Mod
          cur = cur * ((i + 2) / 2) % Mod
          ans = (ans + cur * 2) % Mod
        }
        ans.toInt
      }
    } else {
      0
    }
  }

  class UnionFind(n: Int) {
    private val parent = (0 until n).toArray
    private val sizes = Array.fill[Int](n)(1)
    private var _size = n

    def find(x: Int): Int = {
      if (x == parent(x)) {
        x
      } else {
        parent(x) = find(parent(x))
        parent(x)
      }
    }

    def unite(a: Int, b: Int): Boolean = {
      val fa = find(a)
      val fb = find(b)
      if (fa == fb) {
        false
      } else {
        val (x, y) = if (sizes(fa) >= sizes(fb)) {
          (fa, fb)
        } else {
          (fb, fa)
        }

        parent(y) = x
        sizes(x) += sizes(y)
        sizes(y) = 0

        _size -= 1
        true
      }
    }

    def isSame(x: Int, y: Int): Boolean = find(x) == find(y)

    def partialSizeOf(x: Int): Int = sizes(find(x))

    def size(): Int = _size
  }

  class Combination(max: Int, mod: Int) {
    private val inv = new Array[Long](max + 1)
    private val fact = new Array[Long](max + 1)
    private val invFact = new Array[Long](max + 1)
    inv(1) = 1
    for (i <- 2 to max) inv(i) = inv(mod % i) * (mod - mod / i) % mod
    fact(0) = 1
    invFact(0) = 1
    for (i <- 1 to max) fact(i) = (fact(i - 1) * i) % mod
    for (i <- 1 to max) invFact(i) = (invFact(i - 1) * inv(i)) % mod

    /**
      * get nCm
      */
    def get(n: Int, m: Int): Int = {
      if (n < m) {
        0
      } else fact(n) * invFact(m) % mod * invFact(n - m) % mod
    }.toInt
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
      var stack = List[Int]()
      val added = Array.fill[Boolean](V)(false)
      for {
        i <- used.indices
        if !used(i)
      } {
        stack = i :: stack
        while (stack.nonEmpty) {
          val v = stack.head
          used(v) = true
          var pushed = false
          for {
            u <- graph(v).reverse
            if !used(u)
          } {
            stack = u :: stack
            pushed = true
          }
          if (!pushed) {
            stack = stack.tail
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
        stack = i :: stack
        used(i) = true
        cmp(i) = k
        while (stack.nonEmpty) {
          val v = stack.head
          var pushed = false
          for {
            u <- rg(v)
            if !used(u)
          } {
            used(u) = true
            cmp(u) = k
            stack = u :: stack
            pushed = true
          }
          if (!pushed) {
            stack = stack.tail
          }
        }
        k += 1
      }
      cmp
    }
  }

}
