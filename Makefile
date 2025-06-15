# Try to determine the artifact name. If this does not work replace it with the explicit name.
ARTIFACT := ev3-rust

# Replace this with your ssh configuration for the robot like `robot@192.168.2.3`
TARGET := ev3

all: build

flash: build deploy run
	
build:
	docker run --rm -it -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
    	cargo build --release

deploy:
	scp $(PWD)/target/armv5te-unknown-linux-gnueabi/release/$(ARTIFACT) $(TARGET):.

run:
	ssh $(TARGET) brickrun -r ./$(ARTIFACT)
