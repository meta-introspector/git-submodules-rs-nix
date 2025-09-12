;; Hardcoded Nix-provided Emacs package paths
(add-to-list 'load-path "/nix/store/9j3v5mhah2d7xvzpvkqkm8awy74za86p-emacs-magit-20240426.2118/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/03haksb4hyd6c3vi5c24kwf42vh498j9-emacs-rustic-20230130.912/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/a7jqxkn3nq223mbjj05nla4n67x5sn5b-emacs-cargo-mode-20240116.1949/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/0rb1yrjdxb8azcsfx02cmysbn3aywcz5-emacs-rust-mode-20240415.936/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/pvmb22lhi8n7fsxk17g4z8fqwcw3d2ng-emacs-lsp-mode-20240427.2141/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/dlh2pqdw0fj14w94fj3fqin7r3q47x2n-emacs-company-20240311.1943/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/74jddmpnqb5bvlbg4h593nfc6s6dbzf2-emacs-flycheck-20240411.449/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/w9axsadksn841hi2lmrcjw3x8frdnqp5-emacs-lsp-ui-20240424.412/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/xcypfabw1dx1smjvi8vydlicdfa7dr8k-emacs-dap-mode-20240424.1359/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/nbj1kf2fgmz257qiv6fa9xzw7wpzgrwv-emacs-tuareg-20231009.2143/share/emacs/site-lisp")
(add-to-list 'load-path "/nix/store/dhh42n7g4x7jz9bpixy9fqgq7ps4gqd3-emacs-utop-20220226.1308/share/emacs/site-lisp")

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