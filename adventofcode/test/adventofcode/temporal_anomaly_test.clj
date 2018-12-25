(ns adventofcode.temporal-anomaly-test
    (:require [clojure.test :refer :all]
      [adventofcode.eighteen.temporal-anomaly :refer :all :as core]) (:import (java.time LocalDateTime) (java.time.format DateTimeFormatter)))

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

(deftest parseEventTest
  (testing "[1518-07-04 23:58] Guard #1213 begins shift"
            (is (= (core/parseEvent "[1518-07-04 23:58] Guard #1213 begins shift" {})
                   {:current "#1213"}))
            )
  (testing "[1518-07-04 23:58] Guard #1213 begins shift"
             (is (= (core/parseEvent "[1518-11-22 00:42] falls asleep" {:current "#1213"})
                    {:current "#1213",
                     :sleep (LocalDateTime/parse "1518-11-22 00:42"
                                                 (DateTimeFormatter/ofPattern "yyyy-MM-dd HH:mm"))})
             )
  )
  (testing "[1518-11-22 00:53] wakes up"
           (is (= (core/parseEvent "[1518-11-22 00:53] wakes up"
                                   (parseEvent "[1518-11-22 00:42] falls asleep" {:current "#2141"}))
                  {:current "#2141" :sleep nil :sleepTimes { "#2141" {46 1, 48 1, 50 1, 43 1, 44 1, 51 1, 47 1, 45 1, 53 1, 52 1, 42 1, 49 1}}})
               )
  )
)

(deftest recurUntilEventsAreExhausted
  (testing "Recur until Events are exhausted, return state map"
    (is (= (:current (core/parseEvents {} ["[1518-07-04 23:58] Guard #1213 begins shift"
                                           "[1518-07-05 00:53] falls asleep"
                                           "[1518-07-05 00:58] wakes up"
                                           "[1518-07-06 00:00] Guard #2777 begins shift"
                                           "[1518-07-06 00:25] falls asleep"
                                           "[1518-07-06 00:38] wakes up"] )
             ) "#2777"
           ))
  )
)

(deftest oppositePolarityTest
  (testing "aA annihilates each other"
    (is (= (core/polymer-reactions (apply list (seq "aA"))) ()))
  )
  (testing "abBA annihilates"
    (is (= (core/polymer-reactions (apply list (seq "abBA"))) ()))
  )
  (testing "abAB does not react"
    (is (= (core/polymer-reactions (apply list (seq "abAB"))) '(\B \A \b \a)))
  )
)

(deftest polymer-reaction-test
  (testing "dabAcCaCBAcCcaDA"
    (is (= (core/alchemical-reduction "dabAcCaCBAcCcaDA") "dabCBAcaDA"))
  )
)

(deftest polymer-clean-test
  (testing "clean dabAcCaCBAcCcaDA from a and A"
    (is (= (core/clean-polymer "dabAcCaCBAcCcaDA" \a)) "dbcCCBcCcD")
  )
)

(deftest find-nearest-label-test
  (testing "[1 1] is closest to [1,3]"
    (is (= (core/nearest-label [1 3] [[1, 1]
                                      [1, 6]
                                      [8, 3]
                                      [3, 4]
                                      [5, 5]
                                      [8, 9]])
           [[1 1]])
    )
  )
)