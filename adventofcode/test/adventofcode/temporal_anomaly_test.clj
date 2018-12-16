(ns adventofcode.temporal-anomaly-test
  (:require [clojure.test :refer :all]
    [adventofcode.eighteen.temporal-anomaly :refer :all :as core]))

(deftest letterCountTest
  (testing "abcdef contains no letters that appear exactly two or three times"
    (is (= (core/letterCount "abcdef") [0 0]))
  )
  (testing "abcdee contains two e"
    (is (= (core/letterCount "abcdee") [1 0]))
  )
  (testing "aabcdd contains two a and two d, but only counts once"
    (is (= (core/letterCount "aabcdd") [1 0]))
  )
  (testing "abcccd contains three c, but no letter appears exactly two times."
    (is (= (core/letterCount "abcccd") [0 1]))
  )
  (testing "bababc contains two a and three b, so it counts for both."
    (is (= (core/letterCount "bababc") [1 1]))
  )
)

(deftest letterCountChecksumTest
  (testing "Example [abcdef abcdee aabcdd abcccd bababc]"
    (is (= (core/letterCountChecksum ["abcdef"
                                      "abcdee"
                                      "aabcdd"
                                      "abcccd"
                                      "bababc"]) 6))
  )
)

(deftest compareStringTest
  (testing "compare 'abcde' 'axcye'"
    (is (= (core/hammingDistance "abcde" "axcye") 2))
  )
)

(deftest areaTest
  (testing "#1 @ 1,3: 2x2"
    (is (= (core/area "#1" 1 3 2 2) {[3 1] ["#1"], [4 1] ["#1"], [3 2] ["#1"], [4 2] ["#1"]}))
  )
)

(deftest readClaimTest
  (testing "#1 @ 1,3: 2x2"
    (is (= (core/readClaim "#1 @ 1,3: 2x2") ["#1" 1 3 2 2]))
  )
)

(deftest recurUntilEventsAreExhausted
  (testing "Recur until Events are exhausted, return state map"
    (is (= (core/parseEvents ["[1518-07-04 23:58] Guard #1213 begins shift"
                              "[1518-07-05 00:53] falls asleep"
                              "[1518-07-05 00:58] wakes up"
                              "[1518-07-06 00:00] Guard #2777 begins shift"
                              "[1518-07-06 00:25] falls asleep"
                              "[1518-07-06 00:38] wakes up"] {:i 0})
           {:i 6}))
  )
)