; SPDX-License-Identifier: PMPL-1.0-or-later
;; guix.scm — GNU Guix package definition for formatrix-docs
;; Usage: guix shell -f guix.scm

(use-modules (guix packages)
             (guix build-system gnu)
             (guix licenses))

(package
  (name "formatrix-docs")
  (version "0.1.0")
  (source #f)
  (build-system gnu-build-system)
  (synopsis "formatrix-docs")
  (description "formatrix-docs — part of the hyperpolymath ecosystem.")
  (home-page "https://github.com/hyperpolymath/formatrix-docs")
  (license ((@@ (guix licenses) license) "PMPL-1.0-or-later"
             "https://github.com/hyperpolymath/palimpsest-license")))
