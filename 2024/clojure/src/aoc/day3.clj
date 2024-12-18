(ns aoc.day3
  (:require
   [clojure.string :as str]
   [clojure.test :refer [run-tests is deftest]]))

(defonce test-input
  "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
(defonce input (slurp "src/aoc/day3.txt"))
(defn split [s] (map read-string (str/split s #" +")))

(defn part1 [data-string]
  (->> data-string
       (re-seq #"mul\((\d{1,3}),(\d{1,3})\)")
       (map #(let [a (read-string (nth % 1))
                   b (read-string (nth % 2))]
               (* a b)))
       (reduce +)))
(deftest part1-tests (is (= (part1 test-input) 161)))

(defonce test-input-part2
  "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
(defn part2 [data-string]
  ;; Split by "don't", keeping the data before "don't".
  ;; Split each remaining element by "do()", throwing away data before the "do()"
  (let [[do-first & remaining] (str/split data-string #"don't\(\)")
        do-remaining (map rest (map #(str/split % #"do\(\)") remaining))]
    (reduce +
            (part1 do-first)
            (flatten (map #(map part1 %) do-remaining)))))
(deftest part2-tests (is (= (part2 test-input-part2) 48)))

(run-tests)

(println (part1 input))
(println (part2 input))