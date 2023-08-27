#[cfg(feature = "message")]
pub mod message;
#[cfg(feature = "task")]
pub mod task;

#[cfg(test)]
mod test{
    use crate::task::{AsstTaskParam, StartUpParams};

    #[test]
    fn test_task_param(){
        let param = AsstTaskParam::StartUp(StartUpParams::default());
        assert_eq!(param.get_type(),"StartUp")
    }
}