# sss-computing-strength

sss-computing-strength

Создать и открыть документацию:
cargo doc --no-deps --open  

Расчет изгибающих моментов и срезывающих сил при общем изгибе корпуса судна на тихой воде.

   Общее описание и порядок расчетов для прочности:

   1. Вычисляется общая масса судна путем суммирования всех нагрузок c учетом обледенения. Из общей массы вычисляется объемное водоизмещение.
   2. Перебираются значения осадки и дифферента удовлетворяющие условию совпадения массы судна с массой вытесненной воды и  
   смещения центра тяжести судна со смещением центра объема вытесненной воды.
   3. Из дифферента и средней осадки вычисляется осадка носа и кормы. Из них методом линейной интерполяции вычисляется распределение осадки по каждой шпации.
   4. Вычисляется вытесненная масса воды для каждой шпации.
   5. Вычисляется результирующая сила для каждой шпации как разница веса вытесненной воды и силы тяжести, приходящейся на каждую шпацию.
   6. Вычисляется срезающуя сила ShearForce для каждой шпации через интегрирование. Интегрирование проводим путем вычисления суммы сверху.
   7. Вычисляется изгибающий момент BendingMoment для каждой шпации как интегральнуа  сумма срезающей силы.
  
```mermaid
classDiagram

    class Computer{   
        Класс для 
        расчета прочности
    }
    class BendingMoment{
        Изгибающий момент
    }
    class ShearForce{
        Срезающая сила
    }
    class TotalForce{
        Результирующая нагрузка
        на шпацию
    }
    class Mass{   
        Нагрузка веса на
        корпус судна
    }
    class Volume{
        Распределение объема
        вытесненной воды
        по шпациям
    }
    class Bounds{   
        Разбиение корпуса
        судна по длинне       
    }
    class Displacement{        
        Водоизмещение
    }
    class Draught{
        Осадка судна
    }
    class Trim{   
        Дифферент
    } 
    class Frame{  
        Шпангоут
    }
    class IcingMass{
        Учет обледенения
    }
    class IcingStab{   
        Тип обледенения
    }
    class IcingTimberBound{   
        Ограничение горизонтальной площади 
        обледенения палубного груза - леса
    }
    class WettingMass{
        Учет намокания 
        палубного груза - леса
    }
    class LoadsConst{
        Постоянная масса судна
    }
    class LoadsVariable{
        Все грузы судна
    }
    class Desk{   
        Палубные грузы судна
    }
    class IcingStab{
        Тип обледенения
    }
    class Area{
        Площади горизонтальных
        поверхностей и парусности
    }
    class HAreaStrength{   
        Площадь горизонтальных
        поверхностей корпуса судна
    }
    class VerticalArea{   
        Площадь парусности корпуса судна
    }

    Computer <|-- Mass
            Mass <|-- IcingMass 
                IcingMass <|-- Area
                    Area <|-- VerticalArea
                    Area <|-- HAreaStrength
                    Area <|-- Desk  
                    Area <|-- IcingTimberBound   
                IcingMass <|-- IcingStab
            Mass <|-- WettingMass
            Mass <|-- Bounds
            Mass <|-- LoadsConst
            Mass <|-- LoadsVariable  
    Computer <|-- Displacement
        Displacement <|-- Frame 
    Computer <|-- BendingMoment   
        BendingMoment <|-- ShearForce     
    Computer <|-- ShearForce
        ShearForce <|-- TotalForce
            TotalForce <|-- Mass
            TotalForce <|-- Volume  
                Volume <|-- Displacement
                Volume <|-- Draught
                    Draught <|-- Trim
                        Trim <|-- Mass
                        Trim <|-- Displacement
                        Trim <|-- Bounds
                Volume <|-- Bounds
    Computer <|-- Bounds
```

   Общее описание и порядок расчетов для остойчивости:  

   1. Вычисляется общая масса судна путем суммирования всех нагрузок c учетом обледенения.  
    Из общей массы по кривой водоизмещения с учетом плотности воды вычисляется средняя осадка.
   2. С учетом обледенения и средней осадки вычисляется площадь парусности судна.
   3. Вычисляется статическое давление ветра и момент от смещения центра масс.
   4. Строится диаграммы остойчивости.
   5. Рассчитываются параметры остойчивости.
   6. Для заданного типа судна и условий плавания выбираются и проверяются критерии остойчивости.

```mermaid
classDiagram
    class CriterionComputer{
        Расчет допустимого z_g 
        для критериев остойчивости
    }
    class Criterion{   
        Критерии проверки 
        остойчивости
    }
    class Stability{   
        Расчет критерия погоды К
    }
    class Wind{   
        Расчет плеча кренящего
        момента от давления ветра
    }
    class Windage{   
        Парусность судна
    }
    class Mass{   
        Нагрузка веса на
        корпус судна
    }
    class ShipMoment{
        Момент массы судна
    }
    class IcingMoment{
        Учет обледенения судна
    }
    class WettingMoment{
        Статический момент массы 
        намокания палубного лесного груза
    }
    class LoadsConst{
        Постоянная масса судна
    }
    class LoadsVariable{
        Все грузы судна
    }
    class IcingMass{
        Учет обледенения
    }
    class WettingMass{
        Учет намокания 
        палубного груза - леса
    }
    class Desk{   
        Палубные грузы судна
    }
    class Tank{
        Цистерна с жидкостью
    }
    class Bounds{   
        Разбиение корпуса
        судна по длинне       
    }
    class Area{   
        Момент площади горизонтальных 
        поверхностей и площади 
        парусности
    }
    class HAreaStability{   
        Площадь горизонтальных
        поверхностей корпуса судна
    }
    class IcingStab{   
        Тип обледенения
    }
    class IcingTimberBound{   
        Ограничение горизонтальной площади 
        обледенения палубного груза - леса
    }
    class LeverDiagram{   
        Диаграмма плеч статической
        и динамической остойчивости
    }
    class MetacentricHeight{   
        Продольная и поперечная
        исправленная метацентрическая
        высота
    }
    class RollingAmplitude{   
        Амплитуда качки судна
        с круглой скулой
    }
    class RollingPeriod{   
        Период качки судна
    }
    class Acceleration{   
        Расчет критерия ускорения
    }
    class Circulation{   
        Расчет угла крена
        на циркуляции
    }
    class Grain{   
        Критерий крена от смещения зерна
    }
    class Bulk{   
        Навалочный смещаемый груз
    }
    class DraftMark{
        Расчет уровня заглубления 
        для отметок заглубления
    }
    class Draught{
        Осадка судна
    }
    class Trim{   
        Дифферент
    } 

    CriterionComputer <|-- Criterion
    CriterionComputer <|-- ShipMoment
    CriterionComputer <|-- Mass
    CriterionComputer <|-- Bulk
    CriterionComputer <|-- MetacentricHeight

    Criterion <|-- Grain  
        Grain <|-- Bulk
        Grain <|-- Mass 
            Mass <|-- Bounds        
            Mass <|-- WettingMass
        Grain <|-- LeverDiagram  
    
    Criterion <|-- Circulation
        Circulation <|-- Mass        
        Circulation <|-- ShipMoment  
        Circulation <|-- LeverDiagram

    Criterion <|-- LeverDiagram 
        LeverDiagram <|-- ShipMoment 
            ShipMoment <|-- WettingMoment
            ShipMoment <|-- Mass
                Mass <|-- IcingMass 
                    IcingMass <|-- Area   
                        Area <|-- HAreaStability
                        Area <|-- Desk  
                        Area <|-- IcingTimberBound            
                IcingMass <|-- IcingStab
                Mass <|-- LoadsVariable 
                Mass <|-- LoadsConst                                
            ShipMoment <|-- LoadsVariable            
            ShipMoment <|-- LoadsConst
        LeverDiagram <|-- MetacentricHeight 
                   
    Criterion <|-- Wind
        Wind <|-- Mass  
        Wind <|-- Windage
            Windage <|-- Area
            Windage <|-- IcingStab

    Criterion <|-- Stability
        Stability <|-- LeverDiagram    
        Stability <|-- Wind
        Stability <|-- RollingAmplitude
            RollingAmplitude <|-- MetacentricHeight
            RollingAmplitude <|-- RollingPeriod
            RollingPeriod <|-- MetacentricHeight

    
    Criterion <|-- Acceleration
        Acceleration <|-- RollingAmplitude
        Acceleration <|-- RollingPeriod
        Acceleration <|-- MetacentricHeight         

    Criterion <|-- MetacentricHeight
        MetacentricHeight <|-- ShipMoment
        MetacentricHeight <|-- Mass                
        MetacentricHeight <|-- Tank

    DraftMark <|-- Draught
        Draught <|-- Trim
            Trim <|-- Mass         
            Trim <|-- ShipMoment
                ShipMoment <|-- IcingMoment
                    IcingMoment <|-- Area        
                    IcingMoment <|-- IcingStab                                                          
            Trim <|-- MetacentricHeight
```  
