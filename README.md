# 四則計算

ベーシックな四則計算

## 記述例
```rust
    let inner_parse = |x: &str| calc(x).unwrap();
    assert_eq!(inner_parse("1+1.5"), 2.5);
    assert_eq!(inner_parse("1 +  1.5"), 2.5);
    assert_eq!(inner_parse("32 -23"), 9.0);
    assert_eq!(inner_parse("tan(pi()/3)"), (f64::consts::PI / 3.0).tan());
    assert_eq!(inner_parse("cos(pi()/3)"), (f64::consts::PI / 3.0).cos());
    assert_eq!(inner_parse("sin(pi()/3)"), (f64::consts::PI / 3.0).sin());
    assert_eq!(
        inner_parse("2+4.5*sin(pi()/3)"),
        2.0 + 4.5 * (f64::consts::PI / 3.0).sin()
    );
    assert_eq!(
        inner_parse("2+2*3*(2.4+3)+5/2"),
        2.0 + 2.0 * 3.0 * (2.4 + 3.0) + 5.0 / 2.0
    );
    assert_eq!(
        inner_parse("2+4.5*sin(pi()/2)"),
        2.0 + 4.5 * (f64::consts::PI / 2.0).sin()
    );
    assert_eq!(inner_parse("todegree(asin(sin(torad(60))))"), 60.0);
    assert_eq!(inner_parse("todegree(acos(cos(torad(60))))"), 60.0);
    assert_eq!(inner_parse("todegree(atan(tan(torad(60))))"), 60.0);
    assert_eq!(inner_parse("e()"), f64::consts::E);
    assert_eq!(inner_parse("pi()"), f64::consts::PI);
    assert_eq!(inner_parse("sqrt(4)+sqrt(9)"), 5.0);
    assert_eq!(inner_parse("sqrt(4)*sqrt(9)"), 6.0);
    assert_eq!(inner_parse("3^2+2^3"), 17.0);
    assert_eq!(inner_parse("3^2/sqrt(9)"), 3.0);
```