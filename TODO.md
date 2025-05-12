# TODO

1. [ ] 体力活部分，等哪天课上没事情做就尽早整一下算了
   1. [x] V1的WASM支持，以及一个新的Leptos项目用来跑示例
   2. [ ] V2的WASM支持，以及LeptosOsuapiPlayground的页面修缮，及其对应的后端
   3. [ ] V2的ReqwestAPI，接口、结构与客户端
      1. [x] Changelog
      2. [ ] Chat 较多 chat.read
      3. [ ] Comments 较多
      4. [x] Events
      5. [ ] Forums 较多 forums.read
      6. [x] Home
      7. [x] Matches
      8. [x] Multiplayer 4个
      9. [x] News 2个
      10. [x] Notifications 2个
      11. [x] Ranking 3个
      12. [x] Scores
      13. [x] Users
2. [ ] 即是说，V2还剩下五组API，2小3大，时间分配上，大概是需要一整天时间来完成的
   1. [ ] 其中，chat-comment-forum这三个大的，都还有很多独立的struct，ranking也是，但是multiplayer额外只有一个struct
   2. [ ] 然后的接口编写就很轻松了，今天下午快点完事休息即可，2h左右吧
      1. [x] Interface
      2. [ ] Model 这个是工作量最大的
         1. [x] multiplayer
            1. [x] rooms， 这个文档没写，需要手动测
         2. [x] ranking
         3. [ ] chat
         4. [ ] comment
         5. [ ] forum
      3. [ ] Client 这个主要耗时在测试上了
         1. [x] multiplayer
         2. [x] ranking
         3. [ ] chat
         4. [ ] comment
         5. [ ] forum
   3. [ ] 在这之后，就是
      1. [ ] 修文档
      2. [ ] 修网站，并deno部署
