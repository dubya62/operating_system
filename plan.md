# The Super Elaborate Plan (from boot to user space)

## User keys
-   Each user has a thumb drive with important login information on it
-   Each key also has a customizable boot loader
```
User key:

[custom boot loader (unencrypted)]
[user private key (encrypted by UKD and UPWD)]
```

## The primary drive
-   The kernel is encrypted using a public key (generated at installation)
-   The private key is thrown away, meaning that it is impossible to write to it without someone noticing
```
Primary drive:

[user key decrypter (UKD) (unencrypted)]
[Kernel Decryption Key (KDK) (multiple copies, each encrypted with a user's private key)]
[Kernel (encrypted with KDK's private key)]
[Group Keys (each group key encrypted with each member's public key)]
[User space (Full filesystem encryption with KDK, each file encrypted with group key)]

```

## Booting Process
-   **At each decryption step, verify the integrity of the data before continuing**
-   Before starting the computer, plug in the user key (thumb drive)
-   Boot into the thumb drive (or set thumb drive as primary boot device)
-   The thumb drive will start its customizable bootloader
-   The bootloader will ask for a password (UPWD) and use that and the UKD on the primary drive to decrypt the user private key
-   It will then use the user private key to decrypt its copy of the Kernel Decryption Key
-   It will then use the KDK to decrypt the kernel
-   The bootloader will then jump to the kernel code, passing execution off

## Problems
-   The kernel needs a copy of the root key that it can access without users seeing it
    - The root user would be a member of every group (giving access to every key)
    - This is so that the kernel can run services as different users


## Attacks from outside (logging in)
-   To log in from outside, you would need the KDK, which is encrypted with each user's private key
-   For an attacker to be able to log in, they would need access to at least one user's private key
-   The attacker would then have to get physical access to the machine and enter that user's password
-   The attacker would then be logged in as the user

## Attacks from outside (reading a file)
-   To read a file, the attacker could either log in, or get access to the file's group key and the KDK
-   To get the KDK, the attacker would need to log in as the user
-   To get the group key, the attacker would need the user's public key

## Attacks from outside (writing a file)
-   As long as we check the integrity of files, writing a file will have no effect other than destroying the data
-   The attacker would need everything that is required to read a file to succesfully forge the integrity check

<BR><BR><BR>

## Attacks from users (logging in as somebody else)
-   To log in as somebody else, the attacker would need the other user's private key
-   Obtaining the private key of another user is as difficult for a user as it is for an outside attacker

## Attacks from users (reading somebody else's file)
-   To read somebody else's file, one must have the group key
-   If the attacker is not a member of the group, they do not have a decryptable copy of the group key

## Attacks from users (writing somebody else's file)
-   Writing somebody else's file also requires the group key to succesfully forge the integrity check

<BR><BR><BR>

## Social Engineering (replacing usb with new one)
-   One feasible attack is tricking a user into using a malicious usb
-   The bootloader configuration would look different than the users
-   The user might be tricked into typing in their password
-   If plugged into the main system, the malicious boot loader will have everything needed to get the user's private key ( and decrypt the kernel)

## Social Engineering (modifying user's usb)
-   By modifying the bootloader of the user's usb, the attacker could perform the same trick as when replacing the usb

## Social Engineering (getting user's private key with another computer)
-   The user's private key is encrypted both with information from the main drive and the user's password
-   Extracting the private key without significant levels of access and a weak password are improbable

## Social Engineering (modifying main disk)
-   The user will be unable to decrypt their private key/ KDK/ Kernel/ group keys/ user space
-   As long as integrity checking is working, they will not be fooled

<BR><BR><BR>

## Attack Mitigations
-   Use rolling keys (mitigates attacker get another user's key)
    - every time a user loads the kernel, change their key
        - even if an attacker gets a copy of their key, the next login will change it
    - roll the kernel key every now and then
    - roll user keys every now and then
    - roll keys when user has group priviledges added/revoked
-   Allow admins to require strong enough passwords so that writing over the main disk does not work
    - an attacker could write over the main disk and trick the user into booting on it
    - the user would need to know to not put their password in


## REMAINING PROBLEMS:
-   Prevent a user from being socially engineered when their usb is replaced with an evil twin
-   Give the kernel some way to start up processes (services) as other users without the user getting the root key

