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