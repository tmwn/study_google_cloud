# Cloud Run

## How to set up

Enable continuous deployment following the article
https://cloud.google.com/run/docs/continuous-deployment-with-cloud-build .

Add `SECRET` environment variable from Cloud Run console.

Push the code to GitHub and visit the URL linked from the Cloud Run console.

Visit APIs & Services > Credentials, and confirm the Cilent ID for Study Google Identity.
Set the value to `CLIENT_ID` environment variable.
