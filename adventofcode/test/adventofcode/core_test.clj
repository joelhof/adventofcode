(ns adventofcode.core-test
  (:require [clojure.test :refer :all]
            [adventofcode.core :refer :all :as core]))

(defn stringToVector
  [string]
  (mapv #(Integer/parseInt (str %)) (seq string))  
)  
  
(deftest reverseCaptchaTest
  (testing "1122 is 3"
    (is (= (core/reverseCaptcha [1 1 2 2]) 3)))
  (testing "1111 is 4"
    (is (= (core/reverseCaptcha [1 1 1 1]) 4)))
  (testing "1234 is 0"
    (is (= (core/reverseCaptcha [1 2 3 4]) 0)))
  (testing "91212129 is 9"
    (is (= (core/reverseCaptcha (stringToVector "91212129"))) 9))
)

(deftest reverseCaptchaTestPart2
  (testing "1212 is 6"
    (is (= (core/reverseCaptcha [1 2 1 2] 2) 6)))
  (testing "1221 is 0"
    (is (= (core/reverseCaptcha [1 2 2 1] 2) 0)))
  (testing "123425 is 0"
    (is (= (core/reverseCaptcha [1 2 3 4 2 5] 3) 4)))
  (testing "123123 is 9"
    (is (= (core/reverseCaptcha (stringToVector "123123") 3) 12)))
  (testing "12131415 is 4"
    (is (= (core/reverseCaptcha (stringToVector "12131415") 4) 4)))
)

(deftest checkSumTest
  (testing "5 1 9 5 check sum is 8"
    (is (= (core/rowCheckSum [5 1 9 5]) 8))
  )
  (testing "5 1 9 5, 7 5 3, 2 4 6 8, total checksum is 18"
    (is (= (core/spreadSheetCheckSum core/rowCheckSum [[5 1 9 5] [7 5 3] [2 4 6 8]]) 18))
  )
)

(deftest moduloCheckSumTest
  (testing "5 9 2 8"
    (is (= (core/rowModuloCheckSum [5 9 2 8]) 4))    
  )
  (testing "5 9 2 8"
    (is (= (core/rowModuloCheckSum [9 4 7 3]) 3))
  )
  (testing "5 9 2 8"
    (is (= (core/rowModuloCheckSum [3 8 6 5]) 2))
  )
)

(deftest spiralMemoryTest
  (testing "n = 2"
    (is (= (core/spiralMemory 2) 1))       
  )
  (testing "n = 4"
    (is (= (core/spiralMemory 4) 1))       
  )
)