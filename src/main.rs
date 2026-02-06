use std::ptr;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use screenshots::Screen; // مكتبة التصوير

fn main() {
    let secret_key: u64 = 0x55AA55AA55AA55AA;
    let mut real_data: u64 = 999999; 
    let data_ptr = &mut real_data as *mut u64;
    let emergency_pass = "911"; 

    unsafe {
        ptr::write_volatile(data_ptr, ptr::read_volatile(data_ptr) ^ secret_key);
        println!("📡 [نظام أيجيس - النسخة الاستخباراتية]: نشط وجاهز...");

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            print!("\n🔑 رمز الوصول: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            tx.send(input.trim().to_string()).ok();
        });

        match rx.recv_timeout(Duration::from_secs(15)) {
            Ok(pass) => {
                if pass == emergency_pass {
                    // 1. التوثيق الجنائي (تصوير الشاشة صامتاً)
                    println!("📸 جاري التقاط دليل مرئي للمتسلل...");
                    if let Ok(screens) = Screen::all() {
                        for screen in screens {
                            let image = screen.capture().unwrap();
                            let _ = image.save("Intruder_Evidence.png");
                        }
                    }

                    // 2. تدمير البيانات وإغلاق البرامج
                    ptr::write_volatile(data_ptr, 0); 
                    let _ = Command::new("taskkill").args(&["/F", "/IM", "chrome.exe", "/T"]).spawn();
                    let _ = Command::new("taskkill").args(&["/F", "/IM", "notepad.exe", "/T"]).spawn();

                    // 3. تأثير الماتريكس
                    for _ in 0..30 {
                        println!("1101001010110101010110101010101101010101");
                        thread::sleep(Duration::from_millis(15));
                    }

                    // 4. إشعار النظام
                    let toast = "[void] [System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); $obj = New-Object System.Windows.Forms.NotifyIcon; $obj.Icon = [System.Drawing.SystemIcons]::Shield; $obj.BalloonTipTitle = '⚠️ تم عزل التهديد'; $obj.BalloonTipText = 'تم توثيق المحاولة وتطهير الذاكرة.'; $obj.Visible = $True; $obj.ShowBalloonTip(5000);";
                    let _ = Command::new("powershell").args(&["-Command", toast]).spawn();

                    let _ = Command::new("cmd").args(&["/c", "cls"]).status();
                    println!("🔒 [أيجيس]: تم تأمين النظام. الدليل محفوظ في Intruder_Evidence.png");
                }
            }
            Err(_) => {
                ptr::write_volatile(data_ptr, 0);
                println!("\n⏰ انتهى الوقت.");
            }
        }
    }
}