

source("../../asf_phd/R/eda_startup.r")

farm_disease_states <- read_csv2("../outputs/cattle_farm_outputs.csv")
between_herd_infections <- read_csv2("../outputs/between_herd_infection_events.csv")

farm_disease_states %>%
  pivot_longer(-c(scenario_time, farm_id)) %>% {
  ggplot(., aes(scenario_time, value,
                color = as.factor(farm_id),
                group = interaction(farm_id, name))) +
      # geom_line(show.legend = FALSE) +
      geom_line(show.legend = FALSE) +

      facet_wrap(~name, ncol = 1) +
      NULL
}

between_herd_infections %>%
  pivot_longer(c(origin_farm_id, target_farm_id)) %>% {
  ggplot(., aes(value, name), fill = name) +
      geom_col()
  }
