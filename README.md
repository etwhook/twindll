
# 🎭 TwinDLL

A Dll Proxy / Twin Generator Written Fully in **rust**.



## ❓ How to use
For quick guidance on how to use **TwinDLL** run the following help command.
```
$ ./twindll.exe -h
```
 
## 💻 Compiling
To compile **TwinDLL** , Make sure to have the **rust** compiler and **cargo** installed , Then run the following.
```
cargo build --release
```
## 👀 Examples
```
$ ./twindll.exe -d C:\Windows\System32\advapi32.dll -o main.c -s _bak
```

```
$ ./twindll.exe -d C:\Windows\System32\ntdll.dll -o main.c -s _bak
```
## 🥶 Output
This is a glimpse of the output of TwinDLL when proxying **advapi32.dll**.
```
[+] Proxied -> A_SHAFinal
[+] Proxied -> A_SHAInit
[+] Proxied -> A_SHAUpdate
[+] Proxied -> AbortSystemShutdownA
[+] Proxied -> AbortSystemShutdownW
[+] Proxied -> AccessCheck
[+] Proxied -> AccessCheckAndAuditAlarmA
[+] Proxied -> AccessCheckAndAuditAlarmW
[+] Proxied -> AccessCheckByType
[+] Proxied -> AccessCheckByTypeAndAuditAlarmA
[+] Proxied -> AccessCheckByTypeAndAuditAlarmW
[+] Proxied -> AccessCheckByTypeResultList
.....
```
