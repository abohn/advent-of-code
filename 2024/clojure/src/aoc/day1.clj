(ns aoc.day1
  (:require
   [clojure.string :as str]
   [clojure.test :refer [run-tests is deftest]]))

(def test-input
  "3   4
4   3
2   5
1   3
3   9
3   3")

(defn parse-ints
  [input]
  (for [line (str/split-lines input)] (map bigint (str/split line #" +"))))

(defn transpose
  [m]
  (apply mapv vector m))

(defn part1 [input]
  (reduce
   #(+ (abs %1) (abs %2))
   (apply map - (map sort (transpose (parse-ints input))))))

(deftest part1tests
  (is (= (part1 test-input) 11)))

(part1 (slurp "aoc/day1.txt"))

(run-tests)