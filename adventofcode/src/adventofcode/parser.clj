(ns adventofcode.parser)

"Parser for the garbage infested stream in 2017 Day 9.
Uses the following EBNF grammar in Instaparse notation:
Produces parse tree in Instaparse format
"

(def grammar "S = (expression )*
 expression: group | garbage | string | delete
 garbage: '<' (expression )* '>'
 group: '{' (expression )* '}'
 delete: '!' (char | '{' | '}' | '<' | '>')
 string: (char )*
 char: #\"[^{}<>!]\" ")

(def tokenTypes {
                 "{" :groupStart
                 "}" :groupEnd
                 "<" :garbageStart
                 ">" :garbageEnd
                 :byRegEx { #"^[!]." :delete
                            #"[^{}<>!]" :char
                            #"^[^{}<>!].[^{}<>!]+" :string }
                 }
  )

(defn getTokenType
      [char]
      (if-let [tokenType (tokenTypes char)]
              tokenType
              (->> (tokenTypes :byRegEx)
                  (filter #(re-matches (first %) char) ,,,)
                  (first ,,,)
                  (val ,,,)
              )
      )
)

(defn tokenise
  [s]

)