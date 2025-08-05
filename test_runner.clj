(ns test-runner
  (:require [clojush.interpreter :as interpreter]
            [clojush.pushstate :as pushstate]
            [clojure.data.json :as json]
            [clojure.string :as str]
            ;; Load instruction sets
            [clojush.instructions.numbers]
            [clojush.instructions.boolean]
            [clojush.instructions.common]
            [clojush.instructions.code]
            [clojush.instructions.string]))

;; Run a Push program and extract the final state
(defn run-push-program
  "Runs a Push program and returns the final state as a map"
  [program]
  (let [initial-state (pushstate/make-push-state)
        final-state (interpreter/run-push program initial-state false)]
    {:integer (vec (:integer final-state))
     :float (vec (:float final-state))
     :boolean (vec (:boolean final-state))
     :exec (vec (:exec final-state))
     :code (vec (:code final-state))
     :char (vec (:char final-state))
     :string (vec (:string final-state))}))

;; Convert program string to Clojure data structure
(defn parse-program
  "Parses a Push program string into executable form"
  [program-str]
  (read-string (str "(" program-str ")")))

;; Main function to run tests
(defn -main [& args]
  (if (empty? args)
    (println "Usage: clojure test_runner.clj '<push-program>'")
    (let [program-str (first args)
          program (parse-program program-str)
          result (run-push-program program)]
      (println (json/write-str result)))))

;; Run if called directly
(when (= *file* (System/getProperty "babashka.file"))
  (apply -main *command-line-args*))