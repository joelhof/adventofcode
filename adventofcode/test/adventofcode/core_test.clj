(ns adventofcode.core-test
  (:require [clojure.test :refer :all]
            [adventofcode.core :refer :all :as core]))

(deftest reverseCaptchaTest
  (testing "1122 is 3"
    (is (= (core/reverseCaptcha [1 1 2 2]) 3)))
  (testing "1111 is 4"
    (is (= (core/reverseCaptcha [1 1 1 1]) 4)))
  (testing "1234 is 0"
    (is (= (core/reverseCaptcha [1 2 3 4]) 0)))
  (testing "91212129 is 9"
    (is (= (core/reverseCaptcha (mapv #(Integer/parseInt (str %)) (seq "91212129"))) 9)))
  
)
