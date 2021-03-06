(ns adventofcode.temporal-anomaly-test
    (:require [clojure.test :refer :all]
      [adventofcode.eighteen.temporal-anomaly :refer :all :as core])
    (:require [clojure.string :as string])
    (:import (java.time LocalDateTime) (java.time.format DateTimeFormatter)))

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
    (is (= (core/nearest-labels [1 3] [[1, 1]
                                      [1, 6]
                                      [8, 3]
                                      [3, 4]
                                      [5, 5]
                                      [8, 9]])
           [[1 1]])
    )
  )
  (testing "Tie: [1 6] [3 4] is closest to [0 4]"
    (is (= (core/nearest-labels [0 4] [[1, 1]
                                      [1, 6]
                                      [8, 3]
                                      [3, 4]
                                      [5, 5]
                                      [8, 9]])
                         [[1 6] [3 4]])
      )
    )
)

(deftest find-largest-finite-area-test
  (testing "Example from instructions"
    (is (= (core/largest-area [[1, 1]
                               [1, 6]
                               [8, 3]
                               [3, 4]
                               [5, 5]
                               [8, 9]]) '(9 17)))
  )
)

(deftest total-distance-test
  (testing "The total manhattan distance to all labels from 4,3 is 30"
    (is (= (core/total-distance [4 3] [[1, 1]
                                       [1, 6]
                                       [8, 3]
                                       [3, 4]
                                       [5, 5]
                                       [8, 9]]) 30)
     )
  )
)

(deftest parse-instruction-step-test
  (testing
    (is (= (core/parse-instruction-step
             "Step C must be finished before step A can begin." {})
           {:A nil, :C '(:A)})
    )
  )
)

(def steps "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.")

(deftest test-ready-instructions
  (testing "Only C is ready"
    (is (= (core/ready-steps {:C '(:F :A), :A '(:D :B), :B '(:E), :D '(:E), :F '(:E)})
           '([:C (:F :A)]))
    )
  )
)

(deftest order-instructions-test
  (testing "Example from instructions"
    (is (= (core/instruction-order (string/split-lines steps))
           "CABDFE")
    )
  )
)

(deftest assign-job-test
  (testing "Assign C to W(1,0)"
    (is (= (core/assign-jobs [{[0 0] :C} {}]
                             (reduce #(core/parse-instruction-step %2 %1) {} (string/split-lines steps))
                             1)
           [{[0 0] :C [1 0] :C} {}])
    )
  )
  (testing "Assign A to W(3,0) and F to W(3,1)"
     (is (= (core/assign-jobs [{[0 0] :C [1 0] :C [2 0] :C} {}]
                              {:A '(:D :B), :F '(:E), :B '(:E), :D '(:E), :E nil}
                              3)
            [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {[3 1] :F}])
       )
  )
  (testing "Assign A to W(3,0), only 1 worker is available"
    (with-redefs [core/workers 1]
      (is (= (core/assign-jobs [{[0 0] :C [1 0] :C [2 0] :C} {}]
                               {:A '(:D :B), :F '(:E), :B '(:E), :D '(:E), :E nil}
                               3)
             [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {}])
      )
    )
  )
  (testing "Assign F to W(4,1), and B to W(4,0)"
      (is (= (core/assign-jobs [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {[3 1] :F}]
                               {:F '(:E), :B '(:E), :D '(:E), :E nil}
                               4)
             [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A [4 0] :B} {[3 1] :F [4 1] :F}])
      )
  )
)

(with-redefs [core/task-offset 0
              core/workers 2]
  (deftest remove-finished-jobs-test
    (testing "Dont Remove :C at time 1"
      (is (= (core/remove-finished-jobs {:A '(:D :B), :C '(:F :A), :F '(:E), :B '(:E), :D '(:E), :E nil}
                                        1)
             {:A '(:D :B), :C '(:F :A), :F '(:E), :B '(:E), :D '(:E), :E nil}
          )
      )
    )
    (testing "Remove :C at time=3"
      (is (= (core/remove-finished-jobs {:A '(:D :B), :C {:start 0, :children '(:F :A)}}
                                        63)
             {:A '(:D :B)}))
    )
  )
)

(deftest test-ready-instructions-with-map
  (testing "Only C is ready"
    (is
      (= (core/ready-steps {:C {:start 0, :children '(:F :A)}, :A '(:D :B), :B '(:E), :D '(:E), :F '(:E)})
         '([:C {:start 0, :children (:F :A)}])
      )
    )
  )
)

(deftest test-next-instr
  (testing ":F is next for worker 1"
    (is
      (= (core/next-instr (set (map first (core/ready-steps {:F {:start 3, :children '(:E)}, :B '(:E), :D '(:E), :E nil})))
                                1
                                [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {[3 1] :F}]
                                4)
         :F
      )
    )
  )
  (testing ":B is next for worker 0"
    (is
      (= (core/next-instr (set (map first (core/ready-steps {:F {:start 3, :children '(:E)}, :B '(:E), :D '(:E), :E nil})))
                                0
                                [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {[3 1] :F}]
                                4)
         :B
      )
    )
  )
)

(deftest update-start-times-test
  (testing "Update both :A and :F"
    (is (= (core/update-start-times [{[0 0] :C [1 0] :C [2 0] :C [3 0] :A} {[3 1] :F}]
                                    {:A '(:D :B), :F '(:E), :B '(:E), :D '(:E), :E nil}
                                    3)
           {:A {:start 3 :children '(:D :B)} ,:F {:start 3 :children '(:E)} ,:B '(:E), :D '(:E), :E nil}
        )
    )
  )
)

(deftest parallell-schedule-test
  (testing "Example from instructions"
    (is (= (core/parallell-schedule ["Step C must be finished before step A can begin."])
           [{[0 0] :C, [1 0] :C, [2 0] :C, [3 0] :A}])
    )
  )
  (testing "Example from instructions"
    (is (= (core/parallell-schedule (string/split-lines steps))
           [{[0 0] :C,
             [1 0] :C,
             [11 0] :E,
             [3 0] :A,
             [9 0] :D,
             [13 0] :E,
             [8 0] :D,
             [12 0] :E,
             [10 0] :E,
             [7 0] :D,
             [2 0] :C,
             [5 0] :B,
             [6 0] :D,
             [14 0] :E,
             [4 0] :B}
            {[3 1] :F, [4 1] :F, [5 1] :F, [6 1] :F, [7 1] :F, [8 1] :F}])
    )
  )
)