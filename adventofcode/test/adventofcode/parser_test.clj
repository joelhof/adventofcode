(ns adventofcode.parser-test
  (:require [clojure.test :refer :all]
    [adventofcode.parser :refer :all :as parser]))

(deftest getTokenTypeTest
         (testing "{"
                  (is (= (parser/getTokenType "{") :groupStart))
         )
         (testing "!a"
                  (is (= (parser/getTokenType "!a") :delete))
                  )
)

