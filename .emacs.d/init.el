

;; use-package configuration from terlar-emacs-config
(eval-when-compile
  (require 'use-package))

(eval-and-compile
  (defun use-package-ensure-ignore (&rest _args) t)
  (setq use-package-ensure-function #'use-package-ensure-ignore)

  (setq use-package-always-defer t)
  (setq use-package-hook-name-suffix nil))

;; Assuming init-file-debug is not set, so verbose is nil and expand-minimally is t
(setq use-package-verbose nil
      use-package-expand-minimally t)

;; Early initialization from terlar-emacs-config
(add-hook 'emacs-startup-hook
          (lambda ()
            (message "Loaded Emacs in %.03fs"
                     (float-time (time-subtract after-init-time before-init-time)))))

(let ((normal-gc-cons-threshold gc-cons-threshold)
      (normal-gc-cons-percentage gc-cons-percentage)
      (normal-file-name-handler-alist file-name-handler-alist)
      (init-gc-cons-threshold most-positive-fixnum)
      (init-gc-cons-percentage 0.6))
  (setq gc-cons-threshold init-gc-cons-threshold
        gc-cons-percentage init-gc-cons-percentage
        file-name-handler-alist nil)
  (add-hook 'after-init-hook
            `(lambda ()
               (setq gc-cons-threshold ,normal-gc-cons-threshold
                     gc-cons-percentage ,normal-gc-cons-percentage
                     file-name-handler-alist ',normal-file-name-handler-alist))))

(setq inhibit-startup-echo-area-message t)
(setq inhibit-startup-screen t)
(setq initial-scratch-message nil)

(setq inhibit-default-init t)
(setq initial-major-mode 'fundamental-mode)

(setq package-enable-at-startup nil)

;; UI from terlar-emacs-config
(setq use-dialog-box nil)
(push '(undecorated . t) default-frame-alist)
(push '(menu-bar-lines . 0) default-frame-alist)
(push '(tool-bar-lines . 0) default-frame-alist)
(push '(vertical-scroll-bars) default-frame-alist)

(setq frame-inhibit-implied-resize t)
(defvar global-text-scale-adjust-resizes-frames t)

(push '(drag-internal-border . t) default-frame-alist)

(advice-add #'x-apply-session-resources :override #'ignore)

;; Data configuration from terlar-emacs-config
(defvar data-dir
  (if (getenv "XDG_DATA_HOME")
      (concat (getenv "XDG_DATA_HOME") "/emacs/")
    (expand-file-name "~/.local/share/emacs/"))
  "Directory for data.")

(defvar cache-dir
  (if (getenv "XDG_CACHE_HOME")
      (concat (getenv "XDG_CACHE_HOME") "/emacs/")
    (expand-file-name "~/.cache/emacs/"))
  "Directory for cache.")

(defvar pictures-dir
  (or (getenv "XDG_PICTURES_DIR")
      (expand-file-name "~/Pictures/"))
  "Directory for pictures.")

(setq custom-file (expand-file-name "custom.el" temporary-file-directory))

(use-package no-littering
  :ensure t
  :demand t
  :init
  (setq no-littering-etc-directory data-dir)
  (setq no-littering-var-directory cache-dir))

;; Performance configuration from terlar-emacs-config
(setq auto-mode-case-fold nil)

(setq bidi-inhibit-bpa t)
(setq-default bidi-display-reordering 'left-to-right
              bidi-paragraph-direction 'left-to-right)

(setq fast-but-imprecise-scrolling t)
(setq jit-lock-defer-time 0)

(use-package gcmh
  :ensure t
  :hook
  (after-init-hook . gcmh-mode)
  :init
  (setq gcmh-idle-delay 5)
  (setq gcmh-high-cons-threshold (* 16 1024 1024)) ; 16MB
  (setq gcmh-verbose init-file-debug))

;; Libraries configuration from terlar-emacs-config
(use-package all-the-icons
  :ensure t
  :commands
  (all-the-icons-faicon all-the-icons-octicon))

(require 'cl-lib)
(require 'cl-extra)

;; Functions configuration from terlar-emacs-config
(defmacro quiet! (&rest forms)
  "Run FORMS without making any noise."
  `(if init-file-debug
       (progn ,@forms)
     (let ((message-log-max nil))
       (with-temp-message (or (current-message) "") ,@forms))))

(defun quiet-function-advice (orig-fn &rest args)
  "Advice used to make a function quiet.
Call ORIG-FN with ARGS and suppress the output.  Usage:

  (advice-add \'orig-fn :around #\'quiet-function-advice)"
  (quiet! (apply orig-fn args)))

(defmacro define-repl (fn-name buffer-name command &rest args)
  "Define a REPL function named FN-NAME running COMMAND inside BUFFER-NAME."
  (let ((repl-buffer (concat "*" buffer-name "*")))
    `(defun ,fn-name ()
       ,(format "Run an inferior instance of %s inside Emacs." command)
       (interactive)
       (let ((buffer (get-buffer-create ,repl-buffer)))
         (unless (comint-check-proc ,repl-buffer)
           (apply 'make-comint-in-buffer ,buffer-name buffer ,command nil ,@args))
         (pop-to-buffer buffer)))))

(defun display-ctrl-M-as-newline ()
  "Display `^M' as newline."
  (interactive)
  (setq buffer-display-table (make-display-table))
  (aset buffer-display-table ?^M [?
]))

(defun screenshot (type)
  "Save a screenshot of the current frame as an image in TYPE format.
Saves to a temp file and puts the filename in the kill ring."
  (let* ((ext (concat "." (symbol-name type)))
         (filename (make-temp-file "Emacs-" nil ext))
         (data (x-export-frames nil type)))
    (with-temp-file filename
      (insert data))
    (kill-new filename)
    (message filename)))

(defun screenshot-svg ()
  "Save a screenshot of the current frame as an SVG image.
Saves to a temp file and puts the filename in the kill ring."
  (interactive)
  (screenshot 'svg))

(defun screenshot-png ()
  "Save a screenshot of the current frame as an PNG image.
Saves to a temp file and puts the filename in the kill ring."
  (interactive)
  (screenshot 'png))

(defun send-buffer-to-ssh ()
  "Send the whole buffer to the *ssh* process."
  (interactive)
  (process-send-region "*ssh*" (point-min) (point-max)))

(defun send-to-ssh ()
  "Send selected region or current line to the *ssh* process."
  (interactive)
  (let ((procbuf "*ssh*"))
    (if (use-region-p)
        (process-send-region procbuf (region-beginning) (region-end))
      (process-send-string procbuf (thing-at-point 'line t)))))

(define-minor-mode hide-fringes-mode
  "Toggle hiding of fringes."
  :group 'fringe
  (if hide-fringes-mode
      (set-window-fringes nil 0 0 nil)
    (set-window-fringes nil left-fringe-width right-fringe-width t)))

(define-minor-mode global-hide-fringes-mode
  "Toggle hiding of fringes globally."
  :global t
  :group 'fringe
  (if global-hide-fringes-mode
        (set-fringe-style 0)
      (set-fringe-style nil)))

;; Keybindings configuration from terlar-emacs-config
(defvar leader-key "C-,"
  "The key used for most custom operations.")
(defvar local-leader-key "C-."
  "The key used for major mode operations.")

(defvar toggle-prefix "C-'"
  "Key prefix for commands related to toggling.")
(defvar next-prefix "M-]"
  "Key prefix used for commands doing a next operation.")
(defvar prev-prefix "M-["
  "Key prefix used for commands doing a previous operation.")

(use-package repeat
  :hook
  (after-init-hook . repeat-mode))

;; Editing
(keymap-global-set "<XF86Tools>" #'just-one-space)
(keymap-global-set "C-M-y" #'duplicate-dwim)
(keymap-global-set "C-x C-/" #'revert-buffer)
(keymap-global-set "C-z" #'zap-up-to-char)
(keymap-global-set "<remap> <upcase-word>" #'upcase-dwim)
(keymap-global-set "<remap> <downcase-word>" #'downcase-dwim)
(keymap-global-set "<remap> <capitalize-word>" #'capitalize-dwim)
;; Files
(keymap-global-set "C-x j" #'find-sibling-file)
;; Region
(keymap-global-set "C-x r S" #'sort-lines)

(keymap-set goto-map "k" #'eldoc-doc-buffer)

(use-package window
  :bind
  ((:map window-prefix-map
         ("d" . toggle-dedicated-window)
         ("m" . maximize-window)
         ("r" . window-configuration-to-register)
         ("w" . window-toggle-side-windows))
   (:repeat-map resize-window-repeat-map
                ("+" . balance-windows)
                ("-" . fit-window-to-buffer)
                ("m" . maximize-window))
   (:repeat-map buffer-repeat-map
                ("b" . next-buffer)
                ("B" . previous-buffer)
                ("[" . previous-buffer)
                ("]" . next-buffer))
   (:repeat-map other-window-repeat-map
                ("o" . other-window)
                ("O" . other-window-reverse-repeat)))
  :config
  (defun toggle-dedicated-window ()
    "Toggle selected window as dedicated window."
    (interactive)
    (set-window-dedicated-p (selected-window)
                            (not (window-dedicated-p (selected-window)))))

  (defun other-window-reverse-repeat ()
    (interactive)
    (setq repeat-map 'other-window-repeat-map)
    (other-window -1)))

(defvar-keymap next-map
  :doc "Keymap for the next key sequences."
  :prefix 'next-map-prefix
  "b" '("Buffer" . next-buffer))
(keymap-global-set next-prefix 'next-map-prefix)

(defvar-keymap prev-map
  :doc "Keymap for the prev key sequences."
  :prefix 'prev-map-prefix
  "b" '("Buffer" . previous-buffer))
(keymap-global-set prev-prefix 'prev-map-prefix)

(keymap-global-set toggle-prefix 'toggle-map-prefix)
(defvar-keymap toggle-map
  :doc "Keymap for toggle key sequences."
  :prefix 'toggle-map-prefix
  "*" '("Light/Dark theme" . toggle-theme-mode)
  "c" '("Changes" . highlight-changes-mode)
  "d" '("Debug on error" . toggle-debug-on-error)
  "f" '("Code folding" . hs-minor-mode)
  "F" '("Follow" . follow-mode)
  ;; Group together as one mode?
  "g" '("Sub-word" . subword-mode)
  "G" '("Readable camelCase" . glasses-mode)
  "h" '("Line highlight" . hl-line-mode)
  "l" '("Line numbers" . global-display-line-numbers-mode)
  "t" '("Truncate lines" . toggle-truncate-lines)
  "V" '("Variable-pitch" . variable-pitch-mode)
  "w" '("White-space" . whitespace-mode)
  "x" '("Syntax checker" . flymake-mode))

(keymap-global-set leader-key 'leader-map-prefix)
(defvar-keymap leader-map
  :doc "Keymap for leader key sequences."
  :prefix 'leader-map-prefix
  "m" 'gnus
  "P" 'list-processes
  "s" 'screenshot-svg
  "S" 'screenshot-png
  "x" 'regexp-builder
  "w" 'eww)

(keymap-global-set local-leader-key 'local-leader-map-prefix)
(defvar-keymap local-leader-map
  :doc "Keymap for leader key sequences."
  :prefix 'local-leader-map-prefix)

;; Now you can require the packages you need
(require 'magit)
(require 'rustic)
(require 'cargo-mode)
(require 'rust-mode)
(require 'lsp-mode)
(require 'company)
(require 'flycheck)
(require 'lsp-ui)
(require 'dap-mode)
(require 'tuareg)
(require 'utop)

;; Add any other project-specific Emacs configuration below this line

(require 'magit)
(require 'rustic)
(require 'cargo-mode)
(require 'rust-mode)
(require 'lsp-mode)
(require 'company)
(require 'flycheck)
(require 'lsp-ui)
(require 'dap-mode)
(require 'tuareg)
(require 'utop)

;; Add any other project-specific Emacs configuration below this line