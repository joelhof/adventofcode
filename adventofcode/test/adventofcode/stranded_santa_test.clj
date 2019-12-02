(ns adventofcode.stranded-santa-test
  (:require [clojure.test :refer :all])
  (:require [adventofcode.nineteen.stranded-santa :refer :all as core]))

(deftest integer-computer-opcode-test
  (testing "1,0,0,0,99 =>  2,0,0,0,99"
    (is (= (core/int-code-step [1 0 0 0 99]) [2 0 0 0 99])))
  (testing "2,3,0,3,99 => 2,3,0,6,99"
   (is =) (core/int-code-step [2 3 0 3 99]) [2 3 0 6 99]) )
 
