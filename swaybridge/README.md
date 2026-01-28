# swaybridge - Sway Virtual Output Streaming over SRT

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

A Rust tool for streaming Sway virtual outputs to remote displays over SRT.

## Problem

Your main notebook has maxed out the number of displays but you do want more. A workstation on the same LAN has unused monitors. Goal: extend the notebook's Sway desktop onto the workstation's monitors over the network, controlled entirely from the notebook's keyboard.

You are likely to SSH into the workstation anyway, but setting up tmux sessions there does not scale -- you lose desktop integration. Native Sway output extension gives you full window management, keybindings, and seamless workspace handling across all displays.

## Architecture

```
Notebook (Sway)
+-- Physical monitors (laptop displays)
+-- HEADLESS-1 --> wf-recorder --> SRT :9000 --+
+-- HEADLESS-2 --> wf-recorder --> SRT :9001 --+  AES encrypted
+-- HEADLESS-3 --> wf-recorder --> SRT :9002 --+  H.264 VA-API
                                               |
Workstation (Sway)                             |
+-- DP-1 <-- mpv (fullscreen, view-only) <-----+
+-- DP-2 <-- mpv (fullscreen, view-only) <-----+
+-- DP-3 <-- mpv (fullscreen, view-only) <-----+
```

- Notebook runs Sway with physical + virtual (HEADLESS) outputs
- Virtual outputs are created via `swaymsg create_output`
- Each virtual output is captured and streamed via SRT with H.264 VA-API hardware encoding
- Workstation displays them view-only (no input flows back)
- All window management is native Sway: move windows to any output with keybindings
- Workstation is purely a display -- runs Sway + mpv instances

## Why SRT

- SRT (Secure Reliable Transport, open-sourced 2017) is purpose-built for low-latency streaming
- Built-in AES encryption (128/192/256) via passphrase
- Native H.264/H.265 codec support
- ARQ error correction built in
- Sub-100ms latency achievable on LAN
- Both ffmpeg and mpv support SRT natively

## Caveats

### SRT passphrase visible in process list

The SRT passphrase is passed to ffplay as a URI parameter on the command line,
making it visible via `ps` and `/proc/<pid>/cmdline`. ffplay does not support
reading the passphrase from an environment variable or file. Use network-level
security (VPN/WireGuard) if this is a concern.
