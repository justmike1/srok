use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integration {
    Grafana,
    Jenkins,
    Kibana,
    ArgoCD,
    Prometheus,
    SonarQube,
    OpenSearch,
    Shodan,
    Github,
    GoogleCloud,
    AWS,
    Wifi, // Placeholder
}

impl fmt::Display for Integration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Integration::Grafana => "Grafana",
                Integration::Jenkins => "Jenkins",
                Integration::Kibana => "Kibana",
                Integration::ArgoCD => "ArgoCD",
                Integration::Prometheus => "Prometheus",
                Integration::SonarQube => "SonarQube",
                Integration::OpenSearch => "OpenSearch",
                Integration::Shodan => "Shodan",
                Integration::Github => "Github",
                Integration::GoogleCloud => "Google Cloud",
                Integration::AWS => "AWS",
                Integration::Wifi => "Wifi", // Placeholder
            }
        )
    }
}

impl Integration {
    pub fn is_secret(&self) -> bool {
        matches!(
            self,
            Integration::Shodan | Integration::Github | Integration::GoogleCloud | Integration::AWS
        )
    }

    pub fn is_wifi(&self) -> bool {
        matches!(self, Integration::Wifi)
    }

    pub fn is_tool(&self) -> bool {
        !self.is_secret() && !self.is_wifi()
    }

    pub fn all() -> Vec<Self> {
        vec![
            Integration::Grafana,
            Integration::Jenkins,
            Integration::Kibana,
            Integration::ArgoCD,
            Integration::Prometheus,
            Integration::SonarQube,
            Integration::OpenSearch,
            Integration::Shodan,
            Integration::Github,
            Integration::GoogleCloud,
            Integration::AWS,
            Integration::Wifi, // Placeholder
        ]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().replace(' ', "").as_str() {
            "grafana" => Some(Integration::Grafana),
            "jenkins" => Some(Integration::Jenkins),
            "kibana" => Some(Integration::Kibana),
            "argocd" => Some(Integration::ArgoCD),
            "prometheus" => Some(Integration::Prometheus),
            "sonarqube" => Some(Integration::SonarQube),
            "opensearch" => Some(Integration::OpenSearch),
            "shodan" => Some(Integration::Shodan),
            "github" => Some(Integration::Github),
            "googlecloud" => Some(Integration::GoogleCloud),
            "aws" => Some(Integration::AWS),
            "wifi" => Some(Integration::Wifi), // Placeholder
            _ => None,
        }
    }

    pub fn to_github_query(&self) -> &'static str {
        match self {
            Integration::Shodan => "shodan_api_key",
            Integration::Github => "ghp_",
            Integration::GoogleCloud => "google_api_key",
            Integration::AWS => "aws_secret_access_key",
            _ => panic!("GitHub query not implemented for {}", self),
        }
    }

    pub fn to_shodan_query(&self) -> &'static str {
        match self {
            Integration::Grafana => "product:\"Grafana\"",
            Integration::Jenkins => "X-Jenkins \"Set-Cookie: JSESSIONID\" http.title:\"Dashboard\"",
            Integration::SonarQube => "SonarQube \"HTTP/1.1 200\"",
            Integration::Kibana => "product:\"Kibana\"",
            Integration::ArgoCD => "product:\"ArgoCD\"",
            Integration::Prometheus => "product:\"Prometheus\"",
            Integration::OpenSearch => "product:\"OpenSearch\"",
            _ => panic!("Product query not implemented for {}", self),
        }
    }
}
