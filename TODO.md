文件夹是新建了，Placeholder是写了

简单列一下如果需要完善的话，需要写的内容：

说白了，主要的工作就是繁琐的不断请求、测试、跑案例让这个函数能跑通，就算没问题了

数据模型不能一昧照搬osu!web文档，因为这个文档本身还没完善，以及还被rankHistory和rank_history同时出现给难绷到了一下

此外就是wasm需要放到leptos里面去测，也是留下了充裕的可工作空间了

这个osuapi没有rosu那样巨大的历史包袱，所以生下来就还是很轻便的

但是我目前先放着了，因为osynic需要的只有oauth和user两个部分，我已经实现了v2的not-wasm部分，所以这个库我就先撂在这了www

- [ ] v1
  - [ ] beatmap
  - [ ] user
  - [ ] scores
  - [ ] best
  - [ ] recent
  - [ ] match
  - [ ] replay
- [ ] v2
  - [x] oauth
  - [x] user
  - [ ] beatmap
  - [ ] beatmapset
  - [ ] scores


- [ ] wasm部分，工作量对齐not-wasm部分