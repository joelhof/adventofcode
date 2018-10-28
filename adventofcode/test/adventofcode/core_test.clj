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
  (testing "n = 5"
    (is (= (core/spiralMemory 5) 2))       
  )
  (testing "n = 6"
    (is (= (core/spiralMemory 6) 1))       
  )
  (testing "n = 9"
    (is (= (core/spiralMemory 9) 2))       
  )
  (testing "n = 10"
    (is (= (core/spiralMemory 10) 3))       
  )
  (testing "n = 40"
    (is (= (core/spiralMemory 40) 3))       
  )
)

(deftest spiralMemoryStressTest 
  (testing "n = 2"
    (is (= (core/spiralMemoryStress 2) 4))       
  )
  (testing "n = 23"
    (is (= (core/spiralMemoryStress 23) 25))       
  )
  (testing "n = 26"
    (is (= (core/spiralMemoryStress 26) 54))       
  )
  (testing "n = 747"
    (is (= (core/spiralMemoryStress 747) 806))       
  )
)

(deftest validpassphrase
  (testing "aa bb cc dd ee"
    (is (= (core/isPassphraseValid? "aa bb cc dd ee") 1))         
  )
  (testing "aa bb cc dd aa"
    (is (= (core/isPassphraseValid? "aa bb cc dd aa") 0))         
  )
  (testing "aa bb cc dd aaa"
   (is (= (core/isPassphraseValid? "aa bb cc dd aaa") 1))         
  )
  (testing "oiii ioii iioi iiio"
   (is (= (core/isPassphraseValid? "oiii ioii iioi iiio") 0))         
  )
  (testing "a ab abc abd abf abj"
    (is (= (core/isPassphraseValid? "a ab abc abd abf abj") 1))   
  )
)

(deftest escapeTest
  (testing "escape from 0 [1]"
    (is (= (core/escape? 0 [1]) true))
  )  
)

(deftest instructionJumpTest
  (testing "[0 3  0  1  -3]"
    (is (= (core/jump [0 3  0  1  -3]) 5))      
  )
)

(deftest redistributeTest
  (testing "[0 2 7 0]"
    (is (= (core/redistribute [0 2 7 0]) [2 4 1 2]))         
  )
)

(deftest countReallocateStepsTest
  (testing "[0 2 7 0]"
    (is (= (core/reallocate [0 2 7 0]) [5 4]))        
  )
)

(deftest parseNodeTest
  (testing "pbga (66)"
    (is (= (core/parseNode "pbga (66)") {:name "pbga", :weight "(66)", :children []}))         
  )
  (testing "fwft (72) -> ktlj, cntj, xhth"
    (is (= (core/parseNode "fwft (72) -> ktlj, cntj, xhth")
           {:name "fwft",
            :weight "(72)",
            :children ["ktlj" "cntj" "xhth"]}))         
  )
)  
(deftest findTreeRootTest
  (testing "pbga (66)" "fwft (72) -> pbga" "tknk (41) -> fwft"
    (is (= (core/findRoot ["pbga (66)" "fwft (72) -> pbga" "tknk (41) -> fwft"])
           {:name "tknk"
            :weight "(41)"
            :children ["fwft"]})         
    )
  )
)
(deftest findUnbalancedChildTest
         (testing "padx (45)"
                  "tknk (41) -> ugml, padx"
                  "ugml (68)"
                  (is (= (core/findUnbalancedChild :tknk [68 45]
                                                   {:tknk {:name "tknk", :weight "(41)", :children ["ugml" "padx"]},
                                                    :ugml {:name "ugml", :weight "(68)", :children []},
                                                    :padx {:name "padx", :weight "(45)", :children []}})
                         {:name "padx", :weight "(45)", :children [] :diff -23})
                  )
         )
)
(deftest balanceTreeTest
 (testing "root (5) -> fwft tknk" "fwft (7) -> fgjk klsd" "tknk (15)" "fgjk (5)" "klsd (5)"
  (is (= (core/balanceTree ["root (5) -> fwft, tknk"
                            "fwft (3) -> fgjk, klsd"
                            "tknk (15)"
                            "fgjk (5)"
                            "klsd (5)"])
         {:name "fwft", :weight "(3)", :children ["fgjk" "klsd"] :totalWeight 13 :diff -2})
  )
 )
 (testing "root (5) -> fwft tknk" "fwft (5) -> fgjk klsd" "tknk (15)" "fgjk (5)" "klsd (5)"
  (is (= (core/balanceTree ["root (5) -> fwft, tknk"
                                            "fwft (5) -> fgjk, klsd"
                                            "tknk (15)"
                                            "fgjk (5)"
                                            "klsd (5)"])
         35)
  )
 )
 (testing "root (5) -> fwft tknk" "fwft (7) -> fgjk klsd" "tknk (15)" "fgjk (5)" "klsd (5)"
   (is (= (core/balanceTree ["root (5) -> fwft, tknk, klpn"
                                            "fwft (7) -> fgjk, klsd"
                                            "tknk (15)"
                                            "fgjk (5)"
                                            "klsd (5)"
                                            "klpn (15)"])
          {:name "fwft", :weight "(7)", :children ["fgjk" "klsd"] :totalWeight 17 :diff 2})
   )
 )
 (testing "Test case from website"
   (is (= (core/balanceTree ["pbga (66)"
                             "xhth (57)"
                             "ebii (61)"
                             "havc (66)"
                             "ktlj (57)"
                             "fwft (72) -> ktlj, cntj, xhth"
                             "qoyq (66)"
                             "padx (45) -> pbga, havc, qoyq"
                             "tknk (41) -> ugml, padx, fwft"
                             "jptl (61)"
                             "ugml (68) -> gyxo, ebii, jptl"
                             "gyxo (61)"
                             "cntj (57)"])
          {:name "ugml", :weight "(68)", :children ["gyxo" "ebii" "jptl"], :totalWeight 251, :diff 8})
   )
 )
)

(deftest toPrefixTest
  (testing "a + b -> (+ a b)"
    (is (= (core/toPrefix ["a" "+" "b"]) '(+ (get @r "a" 0) b)))
  )
  (testing "a inc 5 -> (+ a 5)"
   (is (= (core/toPrefix ["a" "inc" "5"]) '(+ (get @r "a" 0) 5)))
  )
  (testing "a dec 5 -> (- a 5)"
    (is (= (core/toPrefix ["a" "dec" "5"]) '(- (get @r "a" 0) 5)))
  )
)

(deftest parseInstructionTest
  (testing "a inc 5 if b > 1"
    (is (= (core/parseInstruction "a inc 5 if b > 1")
           '(if (> (get @r "b" 0) 1) (assoc @r "a" (+ (get @r "a" 0) 5)))
           )
    )
  )
  (testing "a inc 5 if b == 1"
    (is (= (core/parseInstruction "a inc 5 if b == 1")
            '(if (= (get @r "b" 0) 1) (assoc @r "a" (+ (get @r "a" 0) 5)))
        )
    )
  )
)

(deftest evaluateInstructionTest
  (testing "evaluate [{} a inc 5 if b < 1]"
    (is (= (core/evaluateInstruction "a inc 5 if b < 1") {"a" 5}))
  )
  (testing "evaluate [{} a inc 5 if c > 1]"
    (is (= (core/evaluateInstruction "a inc 5 if c > 1") nil))
  )
)