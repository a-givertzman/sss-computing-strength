# sss-computing-strength

sss-computing-strength

Расчет изгибающих моментов и срезывающих сил при общем изгибе корпуса судна на тихой воде.
Подробности расчетов приведены в /design/Статище.
Входные данные:  

- n_parts: количество отрезков разбиения корпуса судна по х,
- water_density: плотность воды,
- center_waterline: кривая отстояния центра тяжести ватерлинии по длине от миделя,
- rad_long: кривая продольного метацентрического радиуса,
- rad_cross: кривая поперечного метацентрического радиуса,
- mean_draught: кривая средней осадки,
- center_draught_shift: кривая отстояния центра величины погруженной части судна,
- массив шпангоутов судна [(index, immersion_area)], где:
  - index: порядковый номер шпангоута,
  - immersion_area: кривая погружаемой площади,
- load_constant: постоянный груз, приходящийся на шпацию
- load_spaces: массив данных по твердым грузам в составе:
  - mass: общая масса груза,
  - bound: границы груза,
  - center: центер масс;
- tanks: массив данных по цистернам в составе:
  - density: плотность жидкости в цистерне,
  - volume: объем жидкости в цистерне,
  - bound: границы цистерны, (x1, x2),
  - center: кривая координат центра объема жидкости в цистерне
         в системе координат судна (volume, x, y, z),
  - free_surf_inertia: кривая момента инерции площади свободной  
         поверхности жидкости (volume, x - поперечный, y - продольный).
   Выходные данные:
- массив значений срезывающих сил,
- массив значений изгибающих моментов,
(TODO: выходные данные для расчета остойчивости)

   Общее описание и порядок расчетов для прочности:

   1. Вычисляется общая масса судна путем суммирования всех нагрузок. Из общей массы по кривой водоизмещения с учетом плотности воды вычисляется объемное водоизмещение $\nabla = \Delta/\rho$.
   2. Перебираются значения дифферента, для этого дифферента выполняются следующие расчеты постепенно приближаясь к нулевому значению изгибающего момента на последней шпации.  
   3. Из дифферента и средней осадки вычисляется осадка носа и кормы. Из них методом линейной интерполяции вычисляется распределение осадки по каждой шпации.
   4. Вычисляется вытесненную массу воды для каждой шпации. Погруженная площадь $S_{start}, S_{end}$ теоретических шпангоутов берется из кривых. $L_{start}, L_{end}$ - расстояние от кормы до шпангоутов, ограничивающих шпацию. Вытесненная масса воды Buoyancy вычисляется как среднее значение погруженной площади умноженное на плотность воды $\gamma$ и на разницу расстояний до теоретических шпангоутов:  
      $V_i = \frac{(S_{start_i} + S_{end_i})}{2}*(L_{end_i}-L_{start_i})*\gamma$
   5. Вычисляется результирующая сила TotalForce для каждой шпации как разницу веса вытесненной воды и массы приходящейся на каждую шпацию, умноженную на гравитационную постоянную g:  
      $Ft_i = (m_i - V_i)*g$.
   6. Вычисляется срезающуя сила ShearForce для каждой шпации через интегрирование. Интегрирование проводим путем вычисления суммы сверху:  
      $Fs_i = Fs_{i-1} + Ft_i, Fs_0 = 0$.
   7. Вычисляется изгибающий момент BendingMoment для каждой шпации как интегральнуа  сумма срезающей силы:  
      $M_i = \left(M_{i-1} + Fs_{i-1} + Fs_i \right)*\frac{\Delta L}{2}, M_0 = 0$.
```mermaid
classDiagram

    class Computer{   
        -gravity_g: f64
        -water_density: f64
        -center_waterline_shift: f64
        -mean_draught: f64
        -mass: Rc~dyn IMass~
        -displacement: Rc~Displacement~
        -bounds: Rc~Bounds~ 
        +bending_moment(&mut self) -> Vec~f64~
        +shear_force(&mut self) -> Vec~f64~
        -calculate(&mut self)
    }

    class BendingMoment{
        shear_force: Box~dyn IShearForce~
        delta: f64
        +values(&mut self) -> Vec~f64~
    }

    class ShearForce{
        total_force: Box~dyn ITotalForce~
        +values(&mut self) -> Vec~f64~ 
    }

    class TotalForce{
        mass: Rc~dyn IMass~
        water_density: f64
        draught: Box~dyn IDraught~
        gravity_g: f64
        +values(&mut self) -> Vec~f64~
    }

    class Mass{
        loads_const: Vec~Rc~Box~dyn ILoad~~~
        shift_const: Position
        loads_cargo: Vec~Rc~Box~dyn ILoad~~~
        bounds: Rc~Bounds~
        +sum(&self) -> f64
        +values(&self) -> Vec~f64~
        +shift(&self) -> Position
        +delta_m_h(&self) -> DeltaMH
        +moment_mass(&self) -> MassMoment
        +moment_surface(&self) -> SurfaceMoment
    }

    class LoadSpace{
        mass: f64
        bound: Bound
        center: Position
        m_f_s_y: f64
        m_f_s_x: f64
        +mass(&self, bound: Option~Bound~) -> f64 
        +center(&self) -> Position
        +moment_surface(&self) -> SurfaceMoment
    }

    class Tank{
        density: f64 
        volume: f64
        bound: Bound
        center: PosShift,
        free_surf_inertia: InertiaShift
        +mass(&self, bound: Option~Bound~) -> f64 
        +center(&self) -> Position
        +moment_surface(&self) -> SurfaceMoment
    }

    class Draught{
        bounds: Rc~Bounds~
        center_waterline_shift: f64
        mean_draught: f64,
        displacement: Rc~Displacement~
        trim: f64,
        +values(&mut self) -> Vec~f64~
    }

    class Bounds{   
        values: Vec~Bound~
        delta: f64
        +from_n(ship_length: f64, n: usize) -> Self 
        +iter(&self) -> std::slice::Iter~'_, Bound~
        +length(&self) -> f64
        +delta(&self) -> f64        
    }

    class Bound{
        start: f64
        end: f64
        +part_ratio(&self, bound: &Bound) -> f64
        +intersect(&self, other: &Bound) -> Option~Bound~
        +length(&self) -> f64
        +start(&self) -> f64
        +end(&self) -> f64
        +center(&self) -> f64
    }

    class Displacement{        
        frames: Vec~Frame~
        +value(&self, bound: Bound, draft_start: f64, draft_end: f64) -> f64
        +area(&self, pos_x: f64, draft: f64) -> f64
    }

    class Frame{  
        shift_x: f64
        area: Curve
        +area(&self, draft: f64) -> f64
        +shift_x(&self) -> f64
    }

    class Curve{
        spline: Spline~f64, f64~
        +new_linear(values: &Vec<(f64, f64)>) -> Curve
        +new_catmull_rom(src: &Vec<(f64, f64)>) -> Curve 
        +value(&self, key: f64) -> f64
        +integral(&self, start: f64, end: f64) -> f64
    }

    Computer <|-- BendingMoment    
    Computer <|-- ShearForce   
    BendingMoment <|-- ShearForce
    ShearForce <|-- TotalForce
    TotalForce <|-- Mass
    TotalForce <|-- Draught
    Mass <|-- LoadSpace
    Mass <|-- Tank
    Mass <|-- Bounds
    Draught <|-- Bounds
    Draught <|-- Displacement
    Displacement <|-- Frame
    Bounds <|-- Bound
    LoadSpace <|-- Bound
    Tank <|-- Bound
    Frame <|-- Curve
```

   Общее описание и порядок расчетов для остойчивости:  

   1. Вычисляется общая масса судна путем суммирования всех нагрузок. Из общей массы по кривой водоизмещения с учетом плотности воды вычисляется объемное водоизмещение $\nabla = \Delta/\rho$.
   2. Исходя из объемного водоизмещения по таблицам элементов теоретического чертежа судна на ровный киль определяются:
      - отстояние центра величины погруженной части судна:
         - по длине от миделя $x_c$;
         - по ширине от ДП $y_c$;
         - по высоте от ОП $z_c$.
      - отстояние центра тяжести ватерлинии по длине от миделя $x_f$;
      - поперечный $r$ и продольный $R$ метацентрические радиусы, м;
      - среднюю осадку $d$;
   Для промежуточных значений определяется линейной интерполяцией. С учетом поправки на влияние свободной поверхности жидкости в цистернах вычисляется дифферент судна.
