//!Расчет изгибающих моментов и срезывающих сил при общем изгибе корпуса судна на тихой воде.
//!Подробности расчетов приведены в [/design/Статище](./../../../design/Статище(3).docx)
//!Входные данные:
//!   - n: количество отрезков разбиения корпуса судна по х,
//!   - water_density: плотность воды,
//!   - ship_length: длинна корпуса судна,
//!   - center_waterline: кривая отстояния центра тяжести ватерлинии по длине от миделя,
//!   - rad_long: кривая продольного метацентрического радиуса,
//!   - mean_draught: кривая средней осадки,
//!   - center_shift: кривая отстояния центра величины погруженной части судна,
//!   - массив шпангоутов судна [(index, immersion_area)], где:
//!      - index: порядковый номер шпангоута,
//!      - immersion_area: кривая погружаемой площади,
//!   - массив данных по твердым грузам в составе:
//!      - mass: общая масса груза,
//!      - bound: границы груза,
//!      - center: центер масс;
//!   - массив данных по цистернам в составе:
//!      - density: плотность жидкости в цистерне,
//!      - volume: объем жидкости в цистерне,
//!      - bound: границы цистерны, (x1, x2, y1, y2),
//!      - center: кривая координат центра объема жидкости в цистерне
//!         в системе координат судна (volume, x, y, z),   
//!      - free_surf_inertia: кривая момента инерции площади свободной  
//!         поверхности жидкости (volume, x - поперечный, y - продольный).
//!   Выходные данные:
//!   - массив значений срезывающих сил,
//!   - массив значений изгибающих моментов.
//!
//!   Общее описание и порядок расчетов:
//!   1. Вычисляется общая масса судна путем суммирования всех нагрузок. Из общей массы по кривой водоизмещения с учетом плотности воды вычисляется объемное водоизмещение $\nabla = \Delta/\rho$.
//!   2. Исходя из объемного водоизмещения по таблицам элементов теоретического чертежа судна на ровный киль определяются:
//!      - отстояние центра величины погруженной части судна:
//!         - по длине от миделя $x_c$;
//!         - по ширине от ДП $y_c$;
//!         - по высоте от ОП $z_c$.
//!      - отстояние центра тяжести ватерлинии по длине от миделя $x_f$;
//!      - поперечный $r$ и продольный $R$ метацентрические радиусы, м;
//!      - среднюю осадку $d$;
//!   Для промежуточных значений определяется линейной интерполяцией. С учетом поправки на влияние свободной поверхности жидкости в цистернах вычисляется дифферент судна.
//!   3. Из дифферента и средней осадки вычисляется осадка носа и кормы. Из них методом линейной интерполяции вычисляется распределение осадки по каждой шпации.
//!   4. Вычисляется вытесненную массу воды для каждой шпации. Погруженная площадь $S_{start}, S_{end}$ теоретических шпангоутов берется из кривых. $L_{start}, L_{end}$ - расстояние от кормы до шпангоутов, ограничивающих шпацию. Вытесненная масса воды Buoyancy вычисляется как среднее значение погруженной площади умноженное на плотность воды $\gamma$ и на разницу расстояний до теоретических шпангоутов: $$V_i = (S_{start_i} + S_{end_i})/2*(L_{end_i}-L_{start_i})*\gamma$$
//!   5. Вычисляется результирующая сила TotalForce для каждой шпации как разницу веса вытесненной воды и массы приходящейся на каждую шпацию, умноженную на гравитационную постоянную g: $Ft_i = (m_i - V_i)*g$.
//!   6. Вычисляется срезающуя сила ShearForce для каждой шпации через интегрирование. Интегрирование проводим путем вычисления суммы сверху: $Fs_i = Fs_{i-1} + Ft_i, Fs_0 = 0$.
//!   7. Вычисляется изгибающий момент BendingMoment для каждой шпации как интегриральнуа сумма срезающей силы:
//!      $M_i = M_{i-1} + Fs_{i-1} + Fs_i, M_0 = 0$.

use std::rc::Rc;

use api_tools::client::{
    api_query::{ApiQuery, ApiQueryKind, ApiQuerySql},
    api_request::ApiRequest,
};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::debug;
use testing::entities::test_value::Value;

use crate::{
    bending_moment::BendingMoment,
    displacement::Displacement,
    draught::Draught,
    frame::Frame,
    load::ILoad,
    mass::{IMass, Mass},
    math::{bound::Bound, curve::Curve, inertia_shift::InertiaShift, pos_shift::PosShift},
    shear_force::{IShearForce, ShearForce},
    tank::Tank,
    total_force::TotalForce,
    trim::Trim,
};

mod bending_moment;
mod data;
mod displacement;
mod draught;
mod frame;
mod load;
mod mass;
mod math;
mod shear_force;
mod tank;
mod tests;
mod total_force;
mod trim;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    debug!("Test the debugging...");
    debug!("Test the testing...");
    let value = Value::Bool(false);
    debug!("\t bool value: {:?}", value);
    let value = Value::Int(444);
    debug!("\t int value: {:?}", value);
    let value = Value::Float(55.55);
    debug!("\t float value: {:?}", value);
    let value = Value::String("66.77".to_string());
    debug!("\t string value: {:?}", value);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new("database", "sql")),
        false,
    );
    let mut request = ApiRequest::new(
        "parent",
        "address",
        "auth_token",
        query.clone(),
        false,
        false,
    );
    let reply = request.fetch(&query, false);

    // длинна судна
    let ship_length = 118.39;
    let n = 20;
    let delta_x = ship_length / n as f64;
    let start_x = -ship_length / 2.;
    // вектор разбиения судна на отрезки
    let bounds = (0..n as usize)
        .map(|v| {
            Bound::new(
                start_x + delta_x * v as f64,
                start_x + delta_x * (v as f64 + 1.),
            )
        })
        .collect::<Vec<_>>();
    // ускорение свободного падения
    let gravity_g = 9.81;
    // плотность окружающей воды
    let water_density = 1.025;
    // отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new(vec![(0., 0.), (10., 1.)]);
    // продольный метацентрический радиус
    let rad_long = Curve::new(vec![(0., 0.), (10., 1.)]);
    // средняя осадка
    let mean_draught = Curve::new(vec![(0., 0.), (1000., 1.), (10000., 10.)]);
    // отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new(vec![(0., 2.), (10., 2.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
    );
    //координаты центра объема жидкости в цистерне в системе координат судна
    let tank_center_draught_shift = PosShift::new(
        Curve::new(vec![(0., 2.), (10., 2.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
    );
    //момент инерции площади свободной поверхности жидкости
    let tank_free_surf_inertia = InertiaShift::new(
        Curve::new(vec![(0., 0.), (10., 1.)]),
        Curve::new(vec![(0., 0.), (10., 1.)]),
    );
    // все грузы судна
    let loads: Vec<Rc<Box<dyn ILoad>>> = vec![Rc::new(Box::new(Tank::new(
        2.,
        10.,
        Bound::new(-5., 5.),
        tank_center_draught_shift,
        tank_free_surf_inertia,
    )))];
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(loads, bounds.clone()));
    let frames = vec![
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
    ];

    let shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        Draught::new(
            ship_length,
            water_density,
            bounds,
            Rc::clone(&mass),
            center_waterline_shift,
            mean_draught,
            Displacement::new(frames, ship_length),
            Trim::new(
                water_density,
                ship_length,
                center_draught_shift, // отстояние центра величины погруженной части судна
                rad_long,             // продольный метацентрические радиус
                Rc::clone(&mass),     // все грузы судна
            ),
        ),
        gravity_g,
    ));
    let bending_moment = BendingMoment::new(&shear_force);
    dbg!(&shear_force.values(), &bending_moment.values());
}
