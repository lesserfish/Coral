# Coral

A NES emulator written in Rust. This is a rewrite of my [Shrimp](https://github.com/lesserfish/Shrimp), originally written in Haskell. 

Unfortunately, Haskell is not as performant as Rust, so I decided to migrate to Rust. Beautiful language by the way.

## Images

We support all of the games originally supported by Shrimp, but with additional mappers. 

![image](https://github.com/user-attachments/assets/39be4dc2-1d96-4a0a-b073-ed2e39367c5e)

![image](https://github.com/user-attachments/assets/6887233a-9654-40b6-8b66-3c5fa8815798)

![image](https://github.com/user-attachments/assets/332dff27-8ff5-454d-b326-4ad06a870f81)

![image](https://github.com/user-attachments/assets/396f009c-21d1-43f3-b641-a7849facc4fe)


## Videos



https://github.com/user-attachments/assets/a1780b3b-6d09-4602-b1e5-7ce15912a772



### Roadmap

Here are some of the roadmaps I have in mind.

- [X] Pass TomHarte tests
- [X] Pass nestest.nes
- [X] Implement background rendering
- [X] Implement sprite rendering
- [X] Implement controls
- [X] Playable
- [X] Implement multithreading for rendering vs emulation
- [ ] Implement Audio
- [ ] Implement Zapper

### Mappers

List of mappers I aim to support one day.

- [X] Mapper 0 (NROM)
- [ ] Mapper 1 (MMC1)
- [X] Mapper 2 (UxROM)
- [ ] Mapper 4 (MMC3)
- [ ] Mapper 5 (MMC5)
- [ ] Mapper 7 (AxROM)
- [ ] Implement more mappers
