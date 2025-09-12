;;; emacs_batch_test.el --- Headless batch test for Emacs configuration -*- lexical-binding: t; -*-

;; This script is designed to be run in Emacs batch mode to verify the
;; loading and basic functionality of the terlar-emacs-config.

(require 'cl-lib)

(defvar test-results '() "List to store test results.")

(defun run-test (name test-fn)
  "Run a test function and record its result."
  (let ((success nil)
        (message ""))
    (condition-case err
        (progn
          (funcall test-fn)
          (setq success t)
          (setq message (format "Test '%s' PASSED." name)))
      (error
       (setq success nil)
       (setq message (format "Test '%s' FAILED: %s" name err))))
    (push (cons success message) test-results)))

(defun summarize-tests ()
  "Print a summary of all test results and exit Emacs."
  (let ((all-passed t))
    (dolist (result (nreverse test-results))
      (let ((success (car result))
            (message (cdr result)))
        (if success
            (message "%s" message)
          (progn
            (message "%s" message)
            (setq all-passed nil)))))
    (if all-passed
        (progn
          (message "\nAll tests passed. Exiting with code 0.")
          (kill-emacs 0))
      (progn
        (message "\nSome tests failed. Exiting with code 1.")
        (kill-emacs 1)))))

;; --- Tests ---

(run-test "Magit Loadable"
  (lambda () 
    (require 'magit)
    (unless (fboundp 'magit-status)
      (error "magit-status function not found"))))

(run-test "Use-Package Loadable"
  (lambda () 
    (require 'use-package)
    (unless (fboundp 'use-package)
      (error "use-package macro not found"))))

(run-test "Corfu Loadable"
  (lambda () 
    (require 'corfu)
    (unless (fboundp 'corfu-mode)
      (error "corfu-mode function not found"))))

;; Add more tests here as needed

;; --- End of Tests ---

;; Run summary after all tests
(summarize-tests)

(provide 'emacs-batch-test)
;;; emacs_batch_test.el ends here
