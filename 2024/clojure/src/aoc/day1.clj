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

;; first attempt
(defn parse-ints
  [input]
  (for [line (str/split-lines input)] (map bigint (str/split line #" +"))))
(defn transpose
  [m]
  (apply mapv vector m))
(defn part1 [s]
  (reduce
   #(+ (abs %1) (abs %2))
   (apply map - (map sort (transpose (parse-ints s))))))

;; rewrite with ->>
(defn split [s] (map read-string (str/split s #" +")))
(defn part1-stream [s]
  (->> (str/split-lines s)
       (map split)
       (apply map list)
       (map sort)
       (apply map -)
       (map abs)
       (reduce +)))

(deftest part1tests
  (is (= (part1 test-input) 11))
  (is (= (part1-stream test-input) 11)))

(defonce file-string (slurp "src/aoc/day1.txt"))
(part1 file-string)

(defn part2 [input]
  (let [[first-col second-col]
        (->> (str/split-lines input)
             (map split)
             (apply map list)
             (map sort))
        second-col-counts (frequencies second-col)]
    (reduce + (map (fn [x] (* x (get second-col-counts x 0))) first-col))))
;;      second-col-counts (frequencies second-col)]

(deftest part2tests
  (is (= (part2 test-input) 31)))
(part2 file-string)

(run-tests)