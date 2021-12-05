init:
	./init_env.sh

proxy:
	bash -c ' \
		[[ -f ./.env ]] || { echo "Run make init" 1>&2; exit 1; }; \
		[[ -f ./cloud_sql_proxy ]] || { echo "Install cloud_sql_proxy" 1>&2; exit 1; }; \
		. ./.env; \
		./cloud_sql_proxy -instances=$${INSTANCE_CONNECTION_NAME}=tcp:5432; \
	'
