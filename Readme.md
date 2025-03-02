# I

接下来就是谱面信息的获取了，因为要显示指定的BeatmapsetItem，并还要做一下缓存

## 暂时用这个测试

```url
https://osu.ppy.sh/oauth/authorize?client_id=36104&redirect_uri=https%3A%2F%2Fapi.osynic.moe%2Fapi%2Fo_oauth&response_type=code&scope=public+identify&state=osynic_ccb
```

卧槽啊，这个还只能挂上去才能测check_state，难绷了
