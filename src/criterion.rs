Критерий погоды
𝐾=𝑏/а 𝐾≥1,

Статический угол крена от действия постоянного ветра

При расчете плеча кренящего момента от давления ветра 𝑙𝑤1, используемое при
определении угла крена θ𝑤1, предполагаемое давление ветра 𝑝𝑣 принимается как для судна
неограниченного района плавания судна. - пересчитать все для этого района

Для всех судов (кроме района плавания R3):
theta_w_1 <= 16.0.min( 0.8*flooding_angle )
Для лесовозов:
theta_w_1 <= 16.0
Для контейнеровозов:
theta_w_1 <= 16.0.min( 0.5*flooding_angle )



Stability
dso_area(&mut self, angle1: f64, angle2: f64) -> Result<f64, Error>

theta_max(&mut self) -> Result<f64, Error> 