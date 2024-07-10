//! Типы судов

/// Типы судов
#[derive(Debug, PartialEq)]
pub enum ShipType {
    /// Навалочное судно
    BulkCarrier,       
    /// Контейнеровоз 
    ContainerShip,    
    /// Суда, предназначенные для перевозки сухих генеральных грузов
    GeneralDryCargoShip, 
    /// Лесовоз
    TimberCarrier,  
    /// Наливное судно    
    Tanker,
    /// Ролкер
    RoRo,
    /// Все остальные типы судов  
    Other,
}
///
impl ShipType {
    /// Конструктор
    /// * chip_type - тип судна
    pub fn new(chip_type: &str) -> Self {
        match chip_type.trim().to_lowercase().as_str() {
            "bulk carrier" => ShipType::BulkCarrier,       
            "container ship" => ShipType::ContainerShip,      
            "general dry cargo ship" => ShipType::GeneralDryCargoShip,   
            "timber carrier" => ShipType::TimberCarrier,    
            "tanker" => ShipType::Tanker,   
            "Ro-ro ship" => ShipType::RoRo,
            _ => ShipType::Other,        
        }
    }
}