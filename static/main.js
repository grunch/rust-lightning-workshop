let interval = null;

$(() => {
  $("#form").collapse("show");
  $("#send-btn").click(sendBtn);
});

const sendBtn = async () => {
  const amount = $("#amount").val();
  const description = $("#description").val();
  $.ajax({
    url: `http://localhost:8000/create_invoice/${description}/${amount}`,
    success: function (invoice) {
      $("#form").collapse("hide");
      $("#invoice-amount").text(amount);
      $("#invoice-text").text(invoice.payment_request);
      $("#invoice").collapse("show");
      $("#success-box").collapse("hide");
      interval = setInterval(waitPayment, 1000, invoice.hash);
    },
    async: false,
  });
};

const waitPayment = async (hash) => {
  $.ajax({
    url: `http://localhost:8000/invoice/${hash}`,
    success: function (invoice) {
      if (invoice.paid) {
        clearInterval(interval);
        interval = null;
        $("#form").collapse("hide");
        $("#invoice").collapse("hide");
        $("#success-box").collapse("show");
      }
    },
    async: false,
  });
};
