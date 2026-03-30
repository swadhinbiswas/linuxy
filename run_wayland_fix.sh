#!/bin/bash
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export GDK_BACKEND=x11
npm run tauri dev
