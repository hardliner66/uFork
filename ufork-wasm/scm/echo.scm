;
; Console I/O echo demonstration (DEVICE 4)
;

;(define echo-in
;    (lambda (io-dev)
;        (BEH (code . error)
;            ; output character-code to console
;            (SEND io-dev (list #? SELF code))
;            ; wait for output to complete
;            (BECOME (echo-out io-dev))
;        )))
;(define echo-out
;    (lambda (io-dev)
;        (BEH (ack)
;            ; input character-code to console
;            (SEND io-dev (list #? SELF))
;            ; wait for input to complete
;            (BECOME (echo-in io-dev))
;        )))
(define echo-beh
    (lambda (io-dev)
        (BEH (code . error)
            (if (eq? #unit code)
                (SEND io-dev (list #? SELF code))   ; output character
                (SEND io-dev (list #? SELF))        ; input character
            )
        )))

(define start
    (lambda (io-dev)
        (SEND
            io-dev
            (list #? (CREATE (echo-beh io-dev))))   ; input character
        ))
(start (DEVICE 4))
