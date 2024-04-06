### WebAssembly smart contract compilation for blockchain execution

Bachelor Project Gustave Charles
Supervised by Gauthier Voron Advised by Prof. Rachid Guerraoui

## Abstract

As blockchains are going more and more mainstream, we need a way to simplify and amplify the experience for the everyday user, but also for the everyday developer. The new ChopChop [1] mechanism has paved the way for a faster consensus layer by multiple orders
of magnitude. This means that we now need to shift our focus on the execution layer. Indeed, the challenge is now to increase the speed of this layer while maintaining and enhancing accessibility. Thus, our focus lies on the creation and integration of a micro-
transactional [2] execution layer that will enable a multi-core bytecode execution, previously unachievable. In this paper, we take a deeper dive and focus on the safe execution of this layer and the detection of malicious code through instrumentation, all while maintaining the current standard of accessibility. We compare state-of-the-art execution with an executable translated into a more suitable security environment and see no performance downside. On the contrary, we observe some promising performances, that
may be shadowed in the future, with more instrumentations and security checks.
