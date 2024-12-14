(ns aoc.day2
  (:require
   [clojure.string :as str]
   [clojure.test :refer [run-tests is deftest]]))

(defonce test-input
  "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")
(defonce input (slurp "src/aoc/day2.txt"))

(defn split [s] (map read-string (str/split s #" +")))
(defn monotonic? [report] (or (apply < report) (apply > report)))
(monotonic? (list 2 1 1))
(deftest mononotic-test
  (is (monotonic? (list 1)))
  (is (monotonic? (list 1 2 3)))
  (is (monotonic? (list 3 2 1)))
  (is (false? (monotonic? (list 2 2 2))))
  (is (false? (monotonic? (list 2 2 1)))))

(defn in-family? [report]
  (reduce (fn [cur next] (let [[l r] next
                               abs-diff (abs (- l r))]
                           (and cur (>= abs-diff 1) (<= abs-diff 3))))
          true
          (partition 2 1 report)))
(deftest in-family-test
  (is (in-family? (list 1 2 3)))
  (is (in-family? (list 3 2 1)))
  (is (in-family? (list 0 3 0)))
  (is (false? (in-family? (list 0 4))))
  (is (false? (in-family? (list 0 -4))))
  (is (false? (in-family? (list 0 0)))))

(defn safe-report? [report] (and (monotonic? report) (in-family? report)))

(deftest safe-report-test
  (is (safe-report? (list 7 6 4 2 1)))
  (is not (safe-report? (list 1 2 7 8 9))))
(in-family? (list 1 2 7 8 9))
(safe-report? (list 1 2 7 8 9))


(defn part1 [data-string]
  (reduce (fn [cnt val] (if val (inc cnt) cnt)) 0
          (->> (str/split-lines data-string)
               (map split)
               (map safe-report?))))
(deftest part1-tests (is (= (part1 test-input) 2)))
(part1 input)



(run-tests)