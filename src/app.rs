use log::{debug, info};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use url::Url;
#[derive(Deserialize, Debug)]
struct TaskRequest {
    id: u8,
}

#[derive(Serialize, Debug)]
struct TaskResponse {
    id: u8,
}

#[post("/", data = "<task>", format = "json")]
fn index(task: Json<TaskRequest>) -> Json<TaskResponse> {
    debug!("task: {:?}", task.id);
    std::process::Command::new("vlc")
        .args(&[
            "--no-video-title-show",
            "--no-video",
            "--quiet",
            "--daemon",
            "-I",
            "http",
            "--http-host",
            "localhost",
            "--http-port",
            "11111",
            "--http-password",
            "secret",
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Couldn't start vlc");
    let u = Url::parse_with_params("http://:secret@localhost:11111/requests/status.xml", &[("command", "in_play"), ("input", "https://r1---sn-a5msen7l.googlevideo.com/videoplayback?expire=1606696430&ei=junDX8_xM8PLkwbx6Lr4Aw&ip=2600%3A8800%3A1300%3A3ae%3Ae18b%3Ae660%3A2219%3A3f46&id=o-ACCVGkp_85uHtDo5UMVFpooFg71n825xh_O8ySJ9Ndjf&itag=251&source=youtube&requiressl=yes&mh=Z6&mm=31%2C29&mn=sn-a5msen7l%2Csn-a5meknzs&ms=au%2Crdu&mv=m&mvi=1&pl=32&initcwndbps=1836250&vprv=1&mime=audio%2Fwebm&ns=osD-A2uWkEejTnsVSKMdWqsF&gir=yes&clen=56986207&dur=3284.001&lmt=1551476330130399&mt=1606674284&fvip=1&keepalive=yes&c=WEB&txp=5511222&n=K8PC-W2wbM95_t9UUq8&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cns%2Cgir%2Cclen%2Cdur%2Clmt&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRgIhAK0JyyQPilGQmuapA_Qx7YmayTtq8mYbrkpvS0e7DhWIAiEAxVUWVrpAuAJnnMkwPpeBcoyIvvod7YCtEPbR2vSHV-I%3D&sig=AOq0QJ8wRQIhAIGFnh9yQzmFp8CspIzJEbFiOkBgbE0EThBWpFW3MuWvAiAFievvxVAaGzutUig-ghFTEP2FIeQjAHCL4YJmyQ2bXA==&ratebypass=yes")]).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let t = reqwest::blocking::get(u)
        .expect("Couldn't http call")
        .text()
        .unwrap();
    info!("{}", t);

    //info!("{:?}", std::str::from_utf8(&c.stdout));
    Json(TaskResponse { id: 0 })
}

pub fn start() {
    rocket::ignite().mount("/", routes![index]).launch();
}
