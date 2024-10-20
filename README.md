get screen capture command of ffmpeg:

build: cargo build --release

output example:

ffmpeg -video_size 895x439 -framerate 25 -f x11grab -i :0.0+612,370 -vf format=yuv420p abc.mp4    //command of recording screen 


pactl list sources short //get your audio device 


ffmpeg -video_size 895x439 -framerate 25 -f x11grab -i :0.0+612,370 -f pulse -i bluez_output.98_DD_60_C6_9E_4F.1.monitor -vf format=yuv420p abc.mp4   //command of recording screen with audio
