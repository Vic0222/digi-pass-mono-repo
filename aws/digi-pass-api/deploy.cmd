aws cloudformation package  --template ./template.yml --s3-bucket digipass.deployment --output-template-file packaged-template.yml
aws cloudformation package  --template ./template.yml --s3-bucket digipass.deployment --output-template-file packaged-template.yml
aws cloudformation create-stack --stack-name digi-pass-api --template-body file://packaged-template.yml --capabilities CAPABILITY_AUTO_EXPAND, CAPABILITY_IAM