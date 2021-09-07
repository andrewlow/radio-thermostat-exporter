#
# https://github.com/andrewlow/radio-thermostat-exporter
#
NAME = exporter-radio-thermostat

# Get secrets
include config.mk

# Create the container
build:
	git pull
	docker build . --tag $(NAME)
	docker create \
		--name=$(NAME) \
		-e TSTAT=$(TSTAT) \
		-p 9864:9864 \
		--restart=unless-stopped \
		$(NAME)

# Start the container
start:
	docker start $(NAME)

# Update the container
update:
	- docker rm $(NAME)-old
	docker rename $(NAME) $(NAME)-old
	make build
	docker stop $(NAME)-old
	make start
