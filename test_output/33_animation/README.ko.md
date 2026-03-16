# 33. 애니메이션 (프레임 추출 / GIF 조립)

애니메이션 GIF에서 개별 프레임을 추출하고, 프레임들을 다시 애니메이션 GIF로 조립하는 테스트.

## 테스트 명령어

```bash
# 애니메이션 GIF → 개별 PNG 프레임 추출
imgconv test_anim.gif --extract-frames --to png -o frames/

# 프레임들을 애니메이션 GIF로 재조립 (프레임당 200ms)
imgconv frames/ --assemble-gif -o reassembled.gif --frame-delay 200
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| test_anim.gif | 원본 애니메이션 GIF (5프레임, 200x150) |
| frames/test_anim_0000.png | 프레임 0 |
| frames/test_anim_0001.png | 프레임 1 |
| frames/test_anim_0002.png | 프레임 2 |
| frames/test_anim_0003.png | 프레임 3 |
| frames/test_anim_0004.png | 프레임 4 |
| reassembled.gif | 추출된 프레임으로 재조립한 애니메이션 GIF |
