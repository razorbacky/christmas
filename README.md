[Rust] 제어 흐름문 : The Twelve Days of Christmas 반복
===

* Rust 공식 문서의 '3.5. 제어 흐름문' 마지막 "정리" 단계에 나온 해보기
* 'The Twelve Days of Christmas' 노래의 반복성을 활용하여 가사 출력해보기

-----

## 알아두기

* "Prefix" , 변수나 값의 접두사 또는 앞에 붙이는 조건적인 수식 (ChatGPT 내용)

  ```rust
  gift(
      gift_day,
      /* prefix */if gift_day == 1 && day != 1 {
          "and"
      } else {
        ""
      },
  );
  ```

1. gift_day == 1 : 현재 선물이 첫번째 선물인지 확인
2. day != 1 첫번째 날이 아닌 경우를 확인
3. 두 조건이 모두 true인 경우 "and" 라는 접두사를 반환한다.

즉, 1절에서는 and를 표시하지 않고, 2절부터 표시하게 만든다.

* **결론**

이 코드에서 언급된 **prefix** 는 선물 텍스트 출력에 붙는 조건부 접두사(and)를 의미한다.
Rust 문법에서 특별한 "prefix 라는 키워드가 아니라, 로직 상 텍스트를 조건 앞에 추가하는 역할.