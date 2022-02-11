use robusta_jni::bridge;

#[bridge]
mod jni {
    use robusta_jni::convert::{Signature, IntoJavaValue, TryIntoJavaValue, TryFromJavaValue, Field};
    use robusta_jni::jni::JNIEnv;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::errors::Error as JniError;
    use tracing::{info, debug};

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(org.meowcat.mesagisto.jni)]
    pub struct MesagistoClient<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }
    impl<'env: 'borrow, 'borrow> MesagistoClient<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {

        }
        pub extern "jni" fn hello(str: String) -> String {
            format!("Hello, {}!", str)
        }
        pub extern "jni" fn initLogger() {
            let env = tracing_subscriber::EnvFilter::from("warn")
                .add_directive("mesagisto_client_jni=trace".parse().unwrap());
            tracing_subscriber::fmt().with_env_filter(env).init();
            debug!("Test logging in rust");
        }
    }

}
